//! Module that contains SMTLib Backend Implementation.
//!
//! This backend outputs the constraints in standard smt-lib2 format. Hence,
//! any solver that supports this format maybe used to solve for constraints.

use std::process::{Child, Command, Stdio};
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::fmt;
use regex::Regex;

use petgraph::graph::{Graph, NodeIndex};
use petgraph::EdgeDirection;

use backends::backend::{Logic, SMTBackend, SMTError, SMTNode, SMTResult};
use theories::{bitvec, core, integer};
use super::backend::SMTRes;

/// Trait that needs to be implemented in order to support a new solver. `SMTProc` is short for
/// "SMT Process".
///
/// To support a new solver that accepts input in the standard SMTLIB2 format, it is sufficient to
/// implement this trait for the struct. This trait describes method needed to spawn, and
/// communicate (read / write) with the solver.
///
/// `read` and `write` methods are implemented by deafult and needs to be changed only if the
/// mode of communication is different (other than process pipes), or if some custom functionality
/// is required for the specific solver.
pub trait SMTProc {
    /// Function to initialize the solver. This includes spawning a process and keeping the process
    /// pipe open for read and write. The function takes &mut self as an argument to allow
    /// configuration during initialization.
    fn init(&mut self);
    /// Return a mutable reference to the process pipe.
    fn pipe<'a>(&'a mut self) -> &'a mut Child;

    fn write<T: AsRef<str>>(&mut self, s: T) -> Result<(), String> {
        // TODO: Check for errors.
        if let Some(ref mut stdin) = self.pipe().stdin.as_mut() {
            stdin.write(s.as_ref().as_bytes()).expect("Write to stdin failed");
            stdin.flush().expect("Failed to flush stdin");
        }
        Ok(())
    }

    fn read(&mut self) -> String {
        // XXX: This read may block indefinitely if there is nothing on the pipe to be
        // read. To prevent this we need a timeout mechanism after which we should
        // return with
        // an error, such as: ErrTimeout.
        // Another important point to note here is that, if the data available to read
        // is exactly
        // 2048 bytes, then this reading mechanism fails and will end up waiting to
        // read more data
        // (when none is available) indefinitely.
        let mut bytes_read = [0; 2048];
        let mut s = String::new();
        let solver = self.pipe();
        if let Some(ref mut stdout) = solver.stdout.as_mut() {
            loop {
                let n = stdout.read(&mut bytes_read).unwrap();
                s = format!("{}{}",
                            s,
                            String::from_utf8(bytes_read[0..n].to_vec()).unwrap());
                if n < 2048 {
                    break;
                }
            }
        }
        s
    }

    // A specific read taylored to output from (check-sat) z3 command
    fn read_checksat_output(&mut self) -> String {
        // Buffer to read into
        let mut buf = String::new();
        // Read from z3's stdout
        if let Some(ref mut stdout) = self.pipe().stdout.as_mut() {
            loop {
                for (_,c) in stdout.bytes().enumerate() {
                    let chr = c.unwrap() as char;
                    buf.push(chr);
                    if buf.ends_with("sat") {
                        return buf;
                    }
                }
            }
        }
        unreachable!()
    }

    // A specific read taylored to output from (get-model) z3 command
    fn read_getmodel_output(&mut self) -> String {
        // Buffer to read into
        let mut buf = String::new();
        // Read from z3's stdout
        if let Some(ref mut stdout) = self.pipe().stdout.as_mut() {
            // Count for paren matching (to detect end of output)
            let mut count = 0;
            loop {
                for (_,c) in stdout.bytes().enumerate() {
                    let chr = c.unwrap() as char;
                    buf.push(chr);
                    match chr {
                        '(' => { count+=1; },
                        ')' => { count-=1; },
                        _ => {}
                    }
                    if count==0 {
                        return buf;
                    }
                }
            }
        }
        unreachable!()
    }

}

#[derive(Clone, Debug)]
pub enum EdgeData {
    EdgeOrder(usize),
}

#[derive(Clone, Debug)]
pub struct SMTLib2<T: Logic> {
    logic: Option<T>,
    gr: Graph<T::Fns, EdgeData>,
    var_index: usize,
    var_map: HashMap<String, (NodeIndex, T::Sorts)>,
    idx_map: HashMap<NodeIndex, String>,
}

impl<L: Logic> SMTLib2<L> {
    pub fn new(logic: Option<L>) -> SMTLib2<L> {
        let mut solver = SMTLib2 {
            logic: logic,
            gr: Graph::new(),
            var_index: 0,
            var_map: HashMap::new(),
            idx_map: HashMap::new(),
        };
        solver
    }

    // Recursive function that builds up the assertion string from the tree.
    pub fn expand_assertion(&self, ni: NodeIndex) -> String {
        let mut children = self.gr
                               .edges_directed(ni, EdgeDirection::Outgoing)
                               .map(|(other, edge)| {
                                   match *edge {
                                       EdgeData::EdgeOrder(ref i) => (other, *i),
                                   }
                               })
                               .collect::<Vec<_>>();
        children.sort_by(|x, y| (x.1).cmp(&y.1));

        let mut assertion = self.gr[ni].to_string();

        assertion = if self.gr[ni].is_fn() {
            format!("({}", assertion)
        } else {
            assertion
        };

        for node in &children {
            assertion = format!("{} {}", assertion, self.expand_assertion(node.0))
        }

        if self.gr[ni].is_fn() {
            format!("{})", assertion)
        } else {
            assertion
        }
    }

    pub fn new_const<T: Into<L::Fns>>(&mut self, cval: T) -> NodeIndex {
        self.gr.add_node(cval.into())
    }
}

impl<L: Logic> SMTBackend for SMTLib2<L> {
    type Idx = NodeIndex;
    type Logic = L;

    fn new_var<T, P>(&mut self, var_name: Option<T>, ty: P) -> Self::Idx
        where T: AsRef<str>,
              P: Into<<<Self as SMTBackend>::Logic as Logic>::Sorts>
    {
        let var_name = var_name.map(|s| s.as_ref().to_owned()).unwrap_or({
            self.var_index += 1;
            format!("X_{}", self.var_index)
        });
        let typ = ty.into();
        let idx = self.gr.add_node(Self::Logic::free_var(var_name.clone(), typ.clone()));
        self.var_map.insert(var_name.clone(), (idx, typ));
        self.idx_map.insert(idx, var_name);
        idx
    }

    fn set_logic<S: SMTProc>(&mut self, smt_proc: &mut S) {
        if self.logic.is_none() {
            return;
        }
        let logic = self.logic.unwrap().clone();
        smt_proc.write(format!("(set-logic {})\n", logic));
    }

    fn assert<T: Into<L::Fns>>(&mut self, assert: T, ops: &[Self::Idx]) -> Self::Idx {
        // TODO: Check correctness like operator arity.
        let assertion = self.gr.add_node(assert.into());
        for (i, op) in ops.iter().enumerate() {
            self.gr.add_edge(assertion, *op, EdgeData::EdgeOrder(i));
        }
        assertion
    }

    fn check_sat<S: SMTProc>(&mut self, smt_proc: &mut S, debug: bool) -> SMTRes {
        // Write out all variable definitions.
        let mut decls = Vec::new();
        for (name, val) in &self.var_map {
            let ni = &val.0;
            let ty = &val.1;
            if self.gr[*ni].is_var() {
                decls.push(format!("(declare-fun {} () {})\n", name, ty));
            }
        }
        // Identify root nodes and generate the assertion strings.
        let mut assertions = Vec::new();
        for idx in self.gr.node_indices() {
            if self.gr.edges_directed(idx, EdgeDirection::Incoming).collect::<Vec<_>>().is_empty() {
                if self.gr[idx].is_fn() {
                    assertions.push(format!("(assert {})\n", self.expand_assertion(idx)));
                }
            }
        }

        for w in decls.iter().chain(assertions.iter()) {
            if debug { print!("{}", w) };
            smt_proc.write(w);
        }

        smt_proc.write("(check-sat)\n".to_owned());
        let read = smt_proc.read_checksat_output();
        if &read == "sat" {
            SMTRes::Sat(read, None)
        } else if &read == "unsat" {
            SMTRes::Unsat(read, None)
        } else {
            SMTRes::Error(read, None)
        }
    }

    // TODO: Return type information along with the value.
    fn solve<S: SMTProc>(&mut self, smt_proc: &mut S, debug: bool) -> (SMTResult<HashMap<Self::Idx, u64>>, SMTRes) {
        let mut result = HashMap::new();
        let check_sat = self.check_sat(smt_proc, debug);

        // If the VC was satisfyable get the model
        match check_sat {
            SMTRes::Sat(ref res, _) => {
                smt_proc.write("(get-model)\n".to_owned());
                // XXX: For some reason we need two reads here in order to get the result from
                // the SMT solver. Need to look into the reason for this. This might stop
                // working in the
                // future.
                let _ = smt_proc.read();
                let read_result = smt_proc.read_getmodel_output();
                let re = Regex::new(r"\s+\(define-fun (?P<var>[0-9a-zA-Z_]+) \(\) [(]?[ _a-zA-Z0-9]+[)]?\n\s+(?P<val>([0-9]+|#x[0-9a-f]+|#b[01]+))")
                             .unwrap();

                /*
                for caps in re.captures_iter(&read_result) {
                    // Here the caps.name("val") can be a hex value, or a binary value or a decimal
                    // value. We need to parse the output to a u64 accordingly.
                    let val_str = caps.name("val").unwrap();
                    let val = if val_str.len() > 2 && &val_str[0..2] == "#x" {
                                  u64::from_str_radix(&val_str[2..], 16)
                              } else if val_str.len() > 2 && &val_str[0..2] == "#b" {
                                  u64::from_str_radix(&val_str[2..], 2)
                              } else {
                                  val_str.parse::<u64>()
                              }
                              .unwrap();
                    let vname = caps.name("var").unwrap();
                    result.insert(self.var_map[vname].0.clone(), val);

                }
                */
                return (Ok(result), SMTRes::Sat(res.clone(), Some(read_result)));
            },
            _ => {}
        }

        (Ok(result), check_sat.clone())
    }
}
