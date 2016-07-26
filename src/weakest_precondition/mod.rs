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
use expression::substitute_variable_in_predicate_with_term;
use expression::{Predicate, Term, BinaryExpressionData, UnaryExpressionData, IntegerBinaryOperator, IntegerUnaryOperator, UnsignedBitVectorData, VariableMappingData, BooleanBinaryOperator, IntegerComparisonOperator, IntegerComparisonData, SignedBitVectorData, BinaryPredicateData};
use rustc::mir::repr::{BasicBlockData, TerminatorKind, Statement, StatementKind, Lvalue, Rvalue, BinOp, UnOp, Operand, Literal, ArgDecl, TempDecl, VarDecl, ProjectionElem};
use rustc::middle::const_val::ConstVal;
use rustc_data_structures::indexed_vec::Idx;

// Computes the weakest precondition for a given postcondition and series of statements over one or more BasicBlocks, both stored in builder
pub fn gen(index: usize, data:&(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>), builder: &Attr) -> Option<Predicate> {
    println!("\n\nExamining bb{:?}\n{:#?}", index, data.1[index]);

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
            println!("\nwp returned as\t{:?}\n", wp.clone().unwrap());
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
    for stmt in stmts {
        // Modify the weakest precondition based on the statement
        wp = gen_stmt(wp.unwrap(), stmt, data);
    }

    // FIXME: Remove debug print statement
    println!("\nwp returned as\t{:?}\n", wp.clone().unwrap());

    // Return the weakest precondition to the preceeding block, or to control
    return wp;
}

// Returns a (possibly) modified weakest precondition based on the content of a statement
pub fn gen_stmt(mut wp: Predicate, stmt: Statement, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>)) -> Option<Predicate>  {
    // FIXME: Remove debug print statement
    println!("processing statement\t{:?}\t\tinto predicate\t{:?}", stmt, wp);

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
            // FIXME: This is a kludge, please fix!
            // Although the checked operators will return a tuple, we will only want to replace the first field of that tuple
            var = VariableMappingData { name: var.name.as_str().to_string() + ".0", var_type: var.var_type.as_str().to_string() };

            let op: IntegerBinaryOperator = match binop {
                &BinOp::Add => {
                    // FIXME: review this with group
                    // get the type of the arguments (assumed to be the same)
                    let ty = match rval.clone() {
                        Operand::Constant(ref c) => { c.ty },
                        _ => { panic!("how did you get here?")}
                    };
                    // create a new weakest precondition representing the origional and the assert
                    let new_wp: Predicate = Predicate::BinaryExpression( BinaryPredicateData{
                        op: BooleanBinaryOperator::And,
                        p1: Box::new(wp),
                        p2: Box::new(Predicate::IntegerComparison( IntegerComparisonData{
                            op: IntegerComparisonOperator::LessThan,
                            // variable we are checking overflow on
                            t1: Box::new(Term::VariableMapping( VariableMappingData {
                                name: var.clone().name,
                                var_type: var.clone().var_type,
                            })),
                            // compute overflow condition
                            t2: Box::new(Term::SignedBitVector( SignedBitVectorData {
                                // size relative to size of type
                                size: match ty.to_string().as_str() {
                                    "i32" => { 32 }
                                    _ => { panic!("unimplemented checkeddAdd argument") }
                                },
                                // value relative to max value of type
                                value: match ty.to_string().as_str() {
                                    "i32" => { i32::max_value() as i64 }
                                    _ => { panic!("unimplemented checkeddAdd argument") }
                                },
                            }))
                        }))
                    } );
                    wp = new_wp;
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
    Some(substitute_variable_in_predicate_with_term( wp, var, term ))
}

// Generates an appropriate variable mapping based on whatever variable, temp, or field is found
pub fn gen_lvalue(lvalue : Lvalue, data : &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>)) -> VariableMappingData {
    match lvalue {
        // Function argument
        Lvalue::Arg(ref arg) => {
            // Find the name and type in the declaration
            VariableMappingData{ name: data.0[arg.index()].debug_name.as_str().to_string(), var_type: data.0[arg.index()].ty.clone().to_string() }
        },
        // Temporary variable
        Lvalue::Temp(ref temp) => {
            // Find the index and type in the declaration
            VariableMappingData{ name: "temp".to_string() + temp.index().to_string().as_str(), var_type: data.2[temp.index()].ty.clone().to_string() }
        },
        // Local variable
        Lvalue::Var(ref var) => {
            // Find the name and type in the declaration
            VariableMappingData{ name: data.3[var.index()].name.to_string(), var_type: data.3[var.index()].ty.clone().to_string() }
        },
        // The returned value
        Lvalue::ReturnPointer => {
            VariableMappingData{ name: "return".to_string(), var_type : "".to_string() }
        },
        // (Most likely) a field of a tuple from a checked operation
        Lvalue::Projection(pro) => {
            // FIXME: This is not shippable code! Only works for one example!
            // Find the name of the tuple, and the index and type of the field

            // FIXME: Lots of intermediaries, should be condensed
            //Trying to get the name of the variable being projected
            println!("projection" );
            let variable: Lvalue = pro.as_ref().base.clone();
            dev_tools::print_type_of(&variable);
            let lvalue_name = match variable {
                Lvalue::Arg(ref arg) => {
                    data.0[arg.index()].debug_name.as_str().to_string()
                },
                Lvalue::Temp(ref temp) => {
                    "temp".to_string() + temp.index().to_string().as_str()
                },
                // Local variable
                Lvalue::Var(ref var) => {
                    // Find the name and type in the declaration
                    data.3[var.index()].name.to_string()
                },
                Lvalue::ReturnPointer => {
                    unimplemented!();
                }
                Lvalue::Static(ref stat) => {
                    unimplemented!();
                }
                Lvalue::Projection(ref proj) => {
                    unimplemented!();
                }
            };

            // FIXME: Lots of intermediaries, should be condensed
            // Trying to get the index
            let proj_index: ProjectionElem<Operand> = pro.as_ref().elem.clone();
            dev_tools::print_type_of(&proj_index);
            //let index_operand: Operand =
            let mut index:String = "".to_string();
            match proj_index {
                ProjectionElem::Index(ref o) => {
                    println!("{:?}", o.clone());
                },
                ProjectionElem::Field(ref field, ref ty) => {
                    println!("{:?} {:?}", field.index(), ty);
                    index = (field.index() as i32).to_string();
                }
                _ => { unimplemented!(); }
            };



            //println!("{:?}", index_operand);
            //Get the index int from index_operand, then stick it in the VariableMappingData

            //let index = ".0";
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
