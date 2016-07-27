// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_const_math;
use super::dev_tools;
use super::Attr;
use super::DEBUG;
use expression::substitute_variable_in_predicate_with_term;
use expression::{Predicate, Term, BinaryExpressionData, UnaryExpressionData, IntegerBinaryOperator, IntegerUnaryOperator, UnsignedBitVectorData, VariableMappingData, BooleanBinaryOperator, IntegerComparisonOperator, IntegerComparisonData, SignedBitVectorData, BinaryPredicateData};
use rustc::mir::repr::{BasicBlockData, TerminatorKind, Statement, StatementKind, Lvalue, Rvalue, BinOp, UnOp, Operand, Literal, ArgDecl, TempDecl, VarDecl, ProjectionElem};
use rustc::middle::const_val::ConstVal;
use rustc_data_structures::indexed_vec::Idx;

// Computes the weakest precondition for a given postcondition and series of statements over one or more BasicBlocks, both stored in builder
pub fn gen(index: usize, data:&(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>), builder: &Attr) -> Option<Predicate> {
    if DEBUG { println!("Examining bb{:?}\n{:#?}\n", index, data.1[index]); }

    //let mut block_targets = Vec::new();
    //let mut block_kind = "";
    let mut wp: Option<Predicate> = None;

    // Parse terminator data
    let terminator = data.1[index].terminator.clone().unwrap().kind;
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => {
            // Retrieve the weakest precondition from the following block
            wp = gen(target.index(), data, builder);
        },
        TerminatorKind::Return => {
            // Return the post condition to the preceeding block
            wp = builder.post_expr.clone();
            return wp;
        },
        TerminatorKind::Goto{target} => {
            // Retrieve the weakest precondition from the following block
            wp = gen(target.index(), data, builder);
        },
        TerminatorKind::Call{func, args, destination, cleanup} => unimplemented!(),
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::If{cond, targets} => { unimplemented!() },
        TerminatorKind::Switch{discr, adt_def, targets} => unimplemented!(),
        TerminatorKind::SwitchInt{discr, switch_ty, values, targets} => unimplemented!(),
    }

    // Examine statements in reverse order
    let mut stmts = data.1[index].statements.clone();
    stmts.reverse();
    if DEBUG { println!("bb{:?}", index);}
    for stmt in stmts {
        // Modify the weakest precondition based on the statement
        wp = gen_stmt(wp.unwrap(), stmt, data);
    }

    // FIXME: Remove debug print statement
    if DEBUG { println!("wp returned as\t{:?}\n", wp.clone().unwrap()); }

    // Return the weakest precondition to the preceeding block, or to control
    return wp;
}

// Returns a (possibly) modified weakest precondition based on the content of a statement
pub fn gen_stmt(mut wp: Predicate, stmt: Statement, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>)) -> Option<Predicate>  {
    // FIXME: Remove debug print statement
    if DEBUG { println!("processing statement\t{:?}\ninto predicate\t\t{:?}", stmt, wp); }

    let mut lvalue: Option<Lvalue> = None;
    let mut rvalue: Option<Rvalue> = None;

    // Store the values of the statement
    match stmt.kind {
        StatementKind::Assign(ref lval, ref rval) => {
            lvalue = Some(lval.clone());
            rvalue = Some(rval.clone());
        }
    }

    // The variable or temp on the left-hand side of the assignment
    let mut var = gen_lvalue(lvalue.unwrap(), data);

    // The term on the right-hand side of the assignment
    let term : Term = match rvalue.clone().unwrap() {
        Rvalue::CheckedBinaryOp(ref binop, ref lval, ref rval) => {
            // FIXME: This probably works for the MIR we encounter, but only time (and testing) will tell
            // Although the checked operators will return a tuple, we will only want to replace the first field of that tuple
            var = VariableMappingData { name: var.name.as_str().to_string() + ".0", var_type: var.var_type.as_str().to_string() };

            let op: IntegerBinaryOperator = match binop {
                &BinOp::Add => {
                    // FIXME: More types may be required
                    // Retrieve the type of the right-hand operand (which should be the same as the left-hand)
                    let ty = match rval.clone() {
                        Operand::Constant(ref c) => { c.ty },
                        Operand::Consume(ref l) => {
                            match l {
                                // Function argument
                                &Lvalue::Arg(ref arg) => {
                                    data.0[arg.index()].ty
                                },
                                // Temporary variable
                                &Lvalue::Temp(ref temp) => {
                                    data.2[temp.index()].ty
                                },
                                // Local variable
                                &Lvalue::Var(ref var) => {
                                    data.3[var.index()].ty
                                },
                                _ => { unimplemented!(); }
                            }
                        }
                    };
                    // Append a clause to the weakest precondition representing the overflow assertion
                    wp = Predicate::BinaryExpression( BinaryPredicateData{
                        op: BooleanBinaryOperator::And,
                        p1: Box::new(wp),
                        p2: Box::new(Predicate::IntegerComparison( IntegerComparisonData{
                            op: IntegerComparisonOperator::LessThan,
                            // Variable we are checking overflow on
                            t1: Box::new(Term::VariableMapping( VariableMappingData {
                                name: var.clone().name,
                                var_type: var.clone().var_type,
                            })),
                            //
                            t2: Box::new(Term::SignedBitVector( SignedBitVectorData {
                                // The bit-vector size of the given type
                                size: match ty.to_string().as_str() {
                                    "i32" => { 32 }
                                    _ => { panic!("unimplemented checkeddAdd right-hand operand type") }
                                },
                                // The maximum value for the given type
                                value: match ty.to_string().as_str() {
                                    "i32" => { i32::max_value() as i64 }
                                    _ => { panic!("unimplemented checkeddAdd right-hand operand type") }
                                },
                            }))
                        }))
                    } );
                    IntegerBinaryOperator::Addition
                },
                &BinOp::Sub => {
                    IntegerBinaryOperator::Subtraction
                },
                &BinOp::Mul => {
                    IntegerBinaryOperator::Multiplication
                },
                &BinOp::Shl => {
                    IntegerBinaryOperator::BitwiseLeftShift
                },
                &BinOp::Shr => {
                    IntegerBinaryOperator::BitwiseRightShift
                },
                _ => {panic!("Unsupported checked binary operation!");}
            };

            let lvalue: Term = gen_operand(&lval, data);
            let rvalue: Term = gen_operand(&rval, data);

            Term::BinaryExpression( BinaryExpressionData {
                op: op,
                t1: Box::new(lvalue),
                t2: Box::new(rvalue)
             } )
        },
        Rvalue::BinaryOp(ref binop, ref lval, ref rval) => {
            let op: IntegerBinaryOperator = match binop {
                &BinOp::Div => {
                    IntegerBinaryOperator::Division
                },
                &BinOp::Rem => {
                    IntegerBinaryOperator::Modulo
                },
                &BinOp::BitOr => {
                    IntegerBinaryOperator::BitwiseOr
                },
                &BinOp::BitAnd => {
                    IntegerBinaryOperator::BitwiseAnd
                },
                &BinOp::BitXor => {
                    IntegerBinaryOperator::BitwiseXor
                },
                _ => {panic!("Unsupported unchecked binary operation!");}
            };

            let lvalue: Term = gen_operand(&lval, data);
            let rvalue: Term = gen_operand(&rval, data);

            Term::BinaryExpression( BinaryExpressionData {
                op: op,
                t1: Box::new(lvalue),
                t2: Box::new(rvalue)
             } )
        },
        Rvalue::UnaryOp(ref unop, ref val) => {
            let op: IntegerUnaryOperator = match unop {
                &UnOp::Not => {
                    IntegerUnaryOperator::BitwiseNot
                },
                &UnOp::Neg => {
                    IntegerUnaryOperator::Negation
                }
            };

            let value: Term = gen_operand(&val, data);

            Term::UnaryExpression( UnaryExpressionData {
                op: op,
                t: Box::new(value)
            } )
        },
        Rvalue::Use(ref operand) => {
            gen_operand(operand, data)
        },
        _ => {panic!("Unsupported RValue type!");}
    };

    // Replace any appearance of var in the weakest precondition with the term
    let ret = Some(substitute_variable_in_predicate_with_term( wp, var, term ));
    if DEBUG { println!("new predicate\t\t{:?}\n---------------------", ret.clone().unwrap());}
    return ret;
}

// Generates an appropriate variable mapping based on whatever variable, temp, or field is found
pub fn gen_lvalue(lvalue : Lvalue, data : &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>)) -> VariableMappingData {
    match lvalue {
        // Function argument
        Lvalue::Arg(ref arg) => {
            // Find the name and type in the declaration
            VariableMappingData{ name: data.0[arg.index()].debug_name.as_str().to_string(), var_type: data.0[arg.index()].ty.clone().to_string() }
            //VariableMappingData{ name: data.0[arg.index()].debug_name.as_str().to_string(), var_type: data.0[arg.index()].ty.clone().to_string() }
        },
        // Temporary variable
        Lvalue::Temp(ref temp) => {
            // Find the index and type in the declaration
            VariableMappingData{ name: "temp".to_string() + temp.index().to_string().as_str(), var_type: data.2[temp.index()].ty.clone().to_string() }
        },
        // Local variable
        Lvalue::Var(ref var) => {
            // Find the name and type in the declaration
            VariableMappingData{ name: "var".to_string() + var.index().to_string().as_str(), var_type: data.3[var.index()].ty.clone().to_string() }
        },
        // The returned value
        Lvalue::ReturnPointer => {
            VariableMappingData{ name: "return".to_string(), var_type : "".to_string() }
        },
        // (Most likely) a field of a tuple from a checked operation
        Lvalue::Projection(pro) => {
            // FIXME: Lots of intermediaries, should be condensed
            // Get the name of the variable being projected
            // FIXME: Remove debug print statement
            let lvalue_name = match pro.as_ref().base {
                // Argument
                Lvalue::Arg(ref arg) => {
                    // Return the name of the argument
                    data.0[arg.index()].debug_name.as_str().to_string()
                },
                // Temporary variable
                Lvalue::Temp(ref temp) => {
                    // Return "temp<index>"
                    "temp".to_string() + temp.index().to_string().as_str()
                },
                // Local variable
                Lvalue::Var(ref var) => {
                    // Return the name of the variable
                    data.3[var.index()].name.to_string()
                },
                Lvalue::ReturnPointer => {
                    unimplemented!();
                }
                Lvalue::Static(ref stat) => {
                    unimplemented!();
                }
                // Multiply-nested projection
                Lvalue::Projection(ref proj) => {
                    unimplemented!();
                }
            };

            // FIXME: Lots of intermediaries, should be condensed
            // Get the index
            let index: String = match pro.as_ref().elem.clone() {
                ProjectionElem::Index(ref o) => {
                    unimplemented!();
                },
                ProjectionElem::Field(ref field, ref ty) => {
                    (field.index() as i32).to_string()
                }
                _ => { unimplemented!(); }
            };

            //Get the index int from index_operand, then stick it in the VariableMappingData
            VariableMappingData{ name: lvalue_name + "." + index.as_str(), var_type: "".to_string() }
        },
        _=> {unimplemented!();}
    }
}

// Generates an appropriate Term based on whatever is found as an operand, either a literal or some kind of variable/temp/field
pub fn gen_operand(operand: &Operand, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>)) -> Term {
    match operand {
        // A variable/temp/field
        &Operand::Consume (ref l) => {
            Term::VariableMapping( gen_lvalue(l.clone(), data) )
        },
        // A literal value
        &Operand::Constant (ref c) => {
            match c.literal {
                Literal::Item {ref def_id, ref substs} => { unimplemented!(); },
                Literal::Value {ref value} => {
                    match value {
                        &ConstVal::Integral(ref const_int) => {
                            Term::UnsignedBitVector( UnsignedBitVectorData { size: 64, value: const_int.to_u64_unchecked() } )
                        },
                        _ => { unimplemented!(); },
                    }
                },
                Literal::Promoted {ref index} => { unimplemented!(); },
            }
        },
    }
}
