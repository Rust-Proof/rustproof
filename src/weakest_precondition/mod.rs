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

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind, LitKind, Attribute_};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::{Span, Spanned};
use syntax::parse::token::intern;
use syntax::ptr::P;
use super::dev_tools; // FIXME: remove for production
use super::Attr;
use super::expression;
use expression::substitute_variable_in_predicate_with_term;
use expression::{Predicate, Term, BinaryExpressionData, UnaryExpressionData, IntegerBinaryOperator, IntegerUnaryOperator, UnsignedBitVectorData, VariableMappingData};
use std::str::FromStr;

use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, TerminatorKind, Statement, StatementKind, Lvalue, Rvalue, BinOp, UnOp, Operand, Literal, ArgDecl, TempDecl, VarDecl};
use rustc::middle::const_val::ConstVal;
use rustc_data_structures::indexed_vec::Idx;
use super::parser;

// computes the weakest precondition
pub fn gen(index: usize, data:&(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>), builder: &mut Attr) -> Option<Predicate> {
    println!("\n\nExamining bb{:?}\n{:#?}", index, data.1[index]);

    //let mut block_targets = Vec::new();
    //let mut block_kind = "";
    let mut wp: Option<Predicate> = None;

    // parse terminator data
    let terminator = data.1[index].terminator.clone().unwrap().kind;
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => {
            wp = gen(target.index(), data, builder);
        },
        TerminatorKind::Return => {
            // return the post condition as expression to start WP gen
            wp = builder.post_expr.clone();
            println!("\nwp returned as\t{:?}\n", wp.clone().unwrap());
            return wp;
        },
        TerminatorKind::Goto{target} => {
            wp = gen(target.index(), data, builder);
        },
        TerminatorKind::Call{func, args, destination, cleanup} => unimplemented!(),
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::If{cond, targets} => unimplemented!(),
        TerminatorKind::Switch{discr, adt_def, targets} => unimplemented!(),
        TerminatorKind::SwitchInt{discr, switch_ty, values, targets} => unimplemented!(),
    }



    // FIXME: add wp generation
    // examine statements in reverse order
    let mut stmts = data.1[index].statements.clone();
    stmts.reverse();
    for stmt in stmts {
        //process stmt into expression
        wp = gen_stmt(wp.unwrap(), stmt, data);
    }

    println!("\nwp returned as\t{:?}\n", wp.clone().unwrap());

    return wp;
}





//FIXME: wp is a predicate but is just a place holder for now. Will need appropriate type in
//       function arguments
pub fn gen_stmt(wp: Predicate, stmt: Statement, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>)) -> Option<Predicate>  {
    println!("processing statement\t{:?}\t\tinto predicate\t{:?}", stmt, wp);

    let mut lvalue: Option<Lvalue> = None;
    let mut rvalue: Option<Rvalue> = None;
    match stmt.kind {
        StatementKind::Assign(ref lval, ref rval) => {
            lvalue = Some(lval.clone());
            rvalue = Some(rval.clone());
        }
    }
    let var = gen_lvalue(lvalue.unwrap());


    //match the rvalue to the correct Rvalue and set term as that new Rvalue
    let term : Term = match rvalue.unwrap() {
        Rvalue::CheckedBinaryOp(ref binop, ref lval, ref rval) => {
            let op: IntegerBinaryOperator = match binop {
                &BinOp::Add => {
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

            let lvalue: Term = gen_operand(&lval);
            let rvalue: Term = gen_operand(&rval);

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

            let lvalue: Term = gen_operand(&lval);
            let rvalue: Term = gen_operand(&rval);

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

            let value: Term = gen_operand(&val);

            Term::UnaryExpression( UnaryExpressionData {
                op: op,
                t: Box::new(value)
            } )
        },
        Rvalue::Use(ref operand) => {
            gen_operand(operand)
        },
        _ => {panic!("Unsupported RValue type!");}
    };

    println!("wp term: {}", term);

    Some(substitute_variable_in_predicate_with_term( wp, var, term ))

}
//FIXME: needs to pass in data as well for arg_data
pub fn gen_lvalue(lvalue : Lvalue) -> VariableMappingData {
    match lvalue {
        //for each match, you need to loop through the appropriate value and find the match
        //then create a new VariableMappingData to hold the name, type of Lvalue
        //data.0 is arg_data
        //data.2 is temp_data
        //data.3 is var_data
        Lvalue::Arg(ref arg) => unimplemented!(),
        Lvalue::Temp(ref temp) => {
            //FIXME:this is ugly fix it
            let index = temp.index().clone();
            let sindex = index.to_string();
            //This line needs to stay:
            let slice_index = sindex.as_str();
            let name = "temp".to_string() + slice_index;
            VariableMappingData{ name: name, var_type: "".to_string()}
        },
        Lvalue::Var(ref var) => unimplemented!(),
        Lvalue::ReturnPointer => VariableMappingData {name: "return".to_string(), var_type : "".to_string()},
        _=> {panic!("what have you done?!");}

    }
}

// For returning a new Term crafted from an operand value.
pub fn gen_operand(operand: &Operand) -> Term {
    match operand {
        &Operand::Consume (ref l) => {
            //FIXME: Use finished LValue parsing code when it's written
            match l {
                &Lvalue::Var(v) => {
                    Term::VariableMapping( VariableMappingData {
                        name: "var".to_string(),
                        var_type: "".to_string()
                    } )
                },
                &Lvalue::Temp(t) => {
                    Term::VariableMapping( VariableMappingData {
                        name: "temp".to_string(),
                        var_type: "".to_string()
                    } )
                },
                &Lvalue::Arg(a) => {
                    Term::VariableMapping( VariableMappingData {
                        name: "arg".to_string(),
                        var_type: "".to_string()
                    } )
                },
                &Lvalue::Static(d) => { unimplemented!(); },
                &Lvalue::ReturnPointer => { unimplemented!(); },
                &Lvalue::Projection(ref b) => {
                    Term::VariableMapping( VariableMappingData {
                        name: "temp.something".to_string(),
                        var_type: "".to_string()
                    } )
                },
            }
        },
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
