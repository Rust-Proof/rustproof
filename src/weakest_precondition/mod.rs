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
use super::DEBUG;
use std::process;
use expression::*;
use rustc::mir::repr::*;
use rustc::middle::const_val::ConstVal;
use rustc_const_math::ConstInt;
use rustc_data_structures::indexed_vec::Idx;
use rustc::ty::{Ty, TypeVariants};
use std::rt::begin_panic_fmt;
use term;


// Computes the weakest precondition for a given postcondition and series of statements over one or more BasicBlocks, both stored in builder
pub fn gen(index: usize, data:&(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>, String), post_expr: &Option<Expression>) -> Option<Expression> {
    // FIXME: Debug should not be a const; it must be user-facing
    if DEBUG { println!("Examining bb{:?}\n{:#?}\n", index, data.1[index]); }

    let mut wp: Option<Expression> = None;

    // Parse terminator data
    let terminator = data.1[index].terminator.clone().unwrap().kind;
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => {
            // Retrieve the weakest precondition from the following block
            wp = gen(target.index(), data, post_expr);
        },
        TerminatorKind::Return => {
            // Return the post condition to the preceeding block
            wp = post_expr.clone();
            return wp;
        },
        TerminatorKind::Goto{target} => {
            // Retrieve the weakest precondition from the following block
            wp = gen(target.index(), data, post_expr);
        },
        TerminatorKind::Call{func, args, destination, cleanup} => {
            // Determine if this is the end of a panic. (assumed false branch of assertion, so return a precondition of false [this path will never be taken])
            match func {
                Operand::Constant (ref c) => {
                    let s = format!("{:?}", c.literal);
                    if s.contains("begin_panic") {
                        return Some(Expression::BooleanLiteral(false));
                    }

                },
                Operand::Consume (ref l) => { unimplemented!() },
            }
            unimplemented!();
        },
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::If{cond, targets} => {
            // Generate weakest precondition for if and else clause
            let wp_if = gen(targets.0.index(), data, post_expr);
            let wp_else = gen(targets.1.index(), data, post_expr);

            // Generate the conditional expression
            let condition = match cond {
                Operand::Constant (ref constant) => {
                    match constant.literal {
                        Literal::Value {ref value} => {
                            match value {
                                &ConstVal::Bool (ref boolean) =>{
                                    Expression::BooleanLiteral(*boolean)
                                },
                                _ => unimplemented!(),
                            }
                        },
                        _ => unimplemented!(),
                    }
                },
                Operand::Consume(c) => {
                    Expression::VariableMapping(gen_lvalue(c, data))
                },
            };
            // Negate the conditional expression
            let not_condition = Expression::UnaryExpression(UnaryExpressionData {op : UnaryOperator::Not, e: Box::new(condition.clone())});
            // wp(If c x else y) = (c -> x) AND ((NOT c) -> y)
            wp = Some(Expression::BinaryExpression(BinaryExpressionData {
                op: BinaryOperator::And,
                left: Box::new(Expression::BinaryExpression(BinaryExpressionData {op: BinaryOperator::Implication, left: Box::new(condition.clone()), right: Box::new(wp_if.unwrap())})),
                right: Box::new(Expression::BinaryExpression(BinaryExpressionData {op: BinaryOperator::Implication, left: Box::new(not_condition.clone()), right: Box::new(wp_else.unwrap())}))
            }));
        },
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

// Returns the type of an operand as a String
pub fn gen_ty(operand: &Operand, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>, String)) -> String {
    match operand.clone() {
        Operand::Constant(ref constant) => { constant.ty.to_string() },
        Operand::Consume(ref lvalue) => {
            match lvalue {
                // Function argument
                &Lvalue::Arg(ref arg) => {
                    data.0[arg.index()].ty.to_string()
                },
                // Temporary variable
                &Lvalue::Temp(ref temp) => {
                    data.2[temp.index()].ty.to_string()
                },
                // Local variable
                &Lvalue::Var(ref var) => {
                    data.3[var.index()].ty.to_string()
                },
                _ => {
                    unimplemented!();
                }
            }
        }
    }
}

//generates an overflow_predicate.
//Option is decided by op.
//If it is BinaryOperator::GreaterThan, it checks the lower bounds
//if it is BinaryOperator::LessThan, it checks the upper bounds
    // FIXME: More types may be required
pub fn gen_overflow_predicate(icop: &BinaryOperator, var: &VariableMappingData ,ty: String) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData {
        op: icop.clone(),
        // Variable we are checking overflow on
        left: Box::new(Expression::VariableMapping( VariableMappingData {
            name: var.clone().name,
            var_type: var.clone().var_type,
        })),
        // Overflow
        right: Box::new(Expression::SignedBitVector( SignedBitVectorData {
            // The bit-vector size of the given type
            size: match ty.as_str() {
                "i8" => { 8 },
                "i16" => { 16 },
                "i32" => { 32 },
                "i64" => { 64 },
                "u8" => { 8 },
                "u16" => { 16 },
                "u32" => { 32 },
                "u64" => { 64 },
                _ => { rp_error!("unimplemented checkeddAdd right-hand operand type") }
            },
        //match on op to see which direction you are detecting overflow in
        value: match icop {
            // if op is GreaterThan check for uppper bounds
            &BinaryOperator::GreaterThan => {
                // The maximum value for the given type
                match ty.as_str() {
                    "i8" => { i8::min_value() as i64 },
                    "i16" => { i16::min_value() as i64 },
                    "i32" => { i32::min_value() as i64 },
                    "i64" => { i64::min_value() as i64 },
                    "u8" => { u8::min_value() as i64 },
                    "u16" => { u16::min_value() as i64 },
                    "u32" => { u32::min_value() as i64 },
                    "u64" => { u64::min_value() as i64 },
                    _ => { panic!("unimplemented checkeddAdd right-hand operand type") }
                }
            },
            // The maximum value for the given type
            &BinaryOperator::LessThan => {
                match ty.as_str() {
                    "i8" => { i8::max_value() as i64 },
                    "i16" => { i16::max_value() as i64 },
                    "i32" => { i32::max_value() as i64 },
                    "i64" => { i64::max_value() as i64 },
                    "u8" => { u8::max_value() as i64 },
                    "u16" => { u16::max_value() as i64 },
                    "u32" => { u32::max_value() as i64 },
                    "u64" => { u64::max_value() as i64 },
                    _ => { panic!("unimplemented checkeddAdd right-hand operand type") }
                }
            },
            // 0 for a div by zero check
            &BinaryOperator::Equal => { 0 },
            _ => { unimplemented!(); }
        }
        }))
    })
}

//generates the upper and lower bounds for overflow check
pub fn gen_overflow_predicate_upper_and_lower(mut wp: Expression, ty: String, var: VariableMappingData) -> Expression {
    let mut v = var;
    v.name = v.name+".0";
    wp = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(wp),
        right: Box::new(gen_overflow_predicate(&BinaryOperator::GreaterThan, &v, ty.clone()))
    } );
    //check the upper bound of overflow
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(wp),
        right: Box::new(gen_overflow_predicate(&BinaryOperator::LessThan, &v, ty.clone()))
    } )
}

//generates a check to make sure that the wp is not divided by 0
pub fn gen_div_zero_check(wp: Expression, ty: String, exp: Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(wp),
        right: Box::new(Expression::BinaryExpression( BinaryExpressionData{
            op: BinaryOperator::NotEqual,
            left: Box::new(exp),
            right: Box::new(Expression::SignedBitVector( SignedBitVectorData {
                // The bit-vector size of the given type
                size: match ty.as_str() {
                    "i8" => { 8 },
                    "i16" => { 16 },
                    "i32" => { 32 },
                    "i64" => { 64 },
                    "u8" => { 8 },
                    "u16" => { 16 },
                    "u32" => { 32 },
                    "u64" => { 64 },
                    _ => { rp_error!("unimplemented checkeddAdd right-hand operand type") }
                },
                value: 0
            }))
        }))
    })
}


// Returns a (possibly) modified weakest precondition based on the content of a statement
pub fn gen_stmt(mut wp: Expression, stmt: Statement, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>, String)) -> Option<Expression>  {
    // FIXME: Remove debug print statement
    if DEBUG { println!("processing statement\t{:?}\ninto expression\t\t{:?}", stmt, wp); }

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

    // The expression on the right-hand side of the assignment
    let mut expression = Vec::new();
    match rvalue.clone().unwrap() {
        Rvalue::CheckedBinaryOp(ref binop, ref loperand, ref roperand) => {
            // FIXME: This probably works for the MIR we encounter, but only time (and testing) will tell
            // Although the checked operators will return a tuple, we will only want to replace the first field of that tuple
            var = VariableMappingData { name: var.name + ".0", var_type: var.var_type };
            let ty = gen_ty(roperand, data);
            let lvalue: Expression = gen_operand(&loperand, data);
            let rvalue: Expression = gen_operand(&roperand, data);
            let op: BinaryOperator = match binop {
                &BinOp::Add => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    BinaryOperator::Addition
                },
                &BinOp::Sub => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    BinaryOperator::Subtraction
                },
                &BinOp::Mul => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    BinaryOperator::Multiplication
                },
                &BinOp::Div => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    // Add the division by 0 expression check
                    wp = gen_div_zero_check(wp, ty.clone(), rvalue.clone());
                    BinaryOperator::Division
                },
                &BinOp::Shl => {
                    BinaryOperator::BitwiseLeftShift
                },
                &BinOp::Shr => {
                    BinaryOperator::BitwiseRightShift
                },
                _ => {rp_error!("Unsupported checked binary operation!");}
            };

            var.name = var.name+".0";

            expression.push(Expression::BinaryExpression( BinaryExpressionData {
                op: op,
                left: Box::new(lvalue),
                right: Box::new(rvalue)
            } ));
        },
        Rvalue::BinaryOp(ref binop, ref lval, ref rval) => {
            let ty = gen_ty(rval, data);
            let lvalue: Expression = gen_operand(&lval, data);
            let rvalue: Expression = gen_operand(&rval, data);
            let op: BinaryOperator = match binop {
                &BinOp::Add => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    BinaryOperator::Addition
                }
                &BinOp::Sub => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    BinaryOperator::Subtraction
                }
                &BinOp::Mul => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    BinaryOperator::Multiplication
                }
                &BinOp::Div => {
                    // Add the overflow and undeflow expression checks
                    wp = gen_overflow_predicate_upper_and_lower(wp, ty.clone(), var.clone());
                    // add the division by 0 expression check
                    wp = gen_div_zero_check(wp, ty.clone(), rvalue.clone());
                    BinaryOperator::Division
                },
                &BinOp::Rem => {
                    // add the division by 0 expression check
                    wp = gen_div_zero_check(wp, ty, rvalue.clone());
                    BinaryOperator::Modulo
                },
                &BinOp::BitOr => {
                    BinaryOperator::BitwiseOr
                },
                &BinOp::BitAnd => {
                    BinaryOperator::BitwiseAnd
                },
                &BinOp::BitXor => {
                    BinaryOperator::BitwiseXor
                },
                &BinOp::Shl => {
                    BinaryOperator::BitwiseLeftShift
                },
                &BinOp::Shr => {
                    BinaryOperator::BitwiseRightShift
                },
                &BinOp::Lt => {
                    BinaryOperator::LessThan
                },
                &BinOp::Le => {
                    BinaryOperator::LessThanOrEqual
                },
                &BinOp::Gt => {
                    BinaryOperator::GreaterThan
                },
                &BinOp::Ge => {
                    BinaryOperator::GreaterThanOrEqual
                },
                &BinOp::Eq => {
                    BinaryOperator::Equal
                },
                &BinOp::Ne => {
                    BinaryOperator::NotEqual
                }
            };

            expression.push(Expression::BinaryExpression( BinaryExpressionData {
                op: op,
                left: Box::new(lvalue),
                right: Box::new(rvalue)
            } ));
        },
        Rvalue::UnaryOp(ref unop, ref val) => {
            let op: UnaryOperator = match unop {
                &UnOp::Not => {
                    UnaryOperator::BitwiseNot
                },
                &UnOp::Neg => {
                    UnaryOperator::Negation
                }
            };

            let value: Expression = gen_operand(&val, data);

            expression.push(Expression::UnaryExpression( UnaryExpressionData {
                op: op,
                e: Box::new(value)
            } ));
        },
        Rvalue::Use(ref operand) => {
            expression.push(gen_operand(operand, data));
        },
        Rvalue::Aggregate(ref ag_kind, ref vec_operand) => {
            match ag_kind {
                &AggregateKind::Tuple => {
                    for i in 0..vec_operand.len() {
                        let e = Expression::VariableMapping( VariableMappingData {
                            //name: var.name.as_str().to_string() + "." + i.to_string().as_str(),
                            name: format!("{:?}", vec_operand[i]),
                            var_type: gen_ty(&vec_operand[i], data)
                        } );
                        expression.push(e);
                    }
                },
                // FIXME: Vectors are weird. let's not bother with them yet
                /*
                &AggregateKind::Vec => {
                    unimplemented!()
                },
                */
                _ => { rp_error!("Unsupported aggregate: only tuples are supported"); }
            }
        },
        Rvalue::Cast(ref cast_kind, ref cast_operand, ref cast_ty) => {
            // FIXME: doesnt do anything
            //println!("cast {:?} {:?} {:?} ", cast_kind, cast_operand, cast_ty);
            //unimplemented!();
            expression.push(Expression::VariableMapping(var.clone()));
        },
        Rvalue::Ref(ref ref_region, ref ref_borrow_kind, ref ref_lvalue) => {
            // FIXME: doesnt do anything
            //println!("ref {:?} {:?} {:?} ", ref_region, ref_borrow_kind, ref_lvalue);
            //unimplemented!();
            expression.push(Expression::VariableMapping(var.clone()));
        },
        Rvalue::Box(..) => { unimplemented!(); },
        Rvalue::Len(..) => { unimplemented!(); },
        _ => { unimplemented!(); }
    };

    // Replace any appearance of var in the weakest precondition with the expression
    for i in 0..expression.len() {
        substitute_variable_with_expression( &mut wp, &var, &expression[i] );
    }
    if DEBUG { println!("new expression\t\t{:?}\n---------------------", wp.clone());}
    return Some(wp);
}

// Generates an appropriate variable mapping based on whatever variable, temp, or field is found
pub fn gen_lvalue(lvalue : Lvalue, data : &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>, String)) -> VariableMappingData {
    match lvalue {
        // Function argument
        Lvalue::Arg(ref arg) => {
            // Find the name and type in the declaration
            VariableMappingData{ name: data.0[arg.index()].debug_name.as_str().to_string(), var_type: data.0[arg.index()].ty.clone().to_string() }
        },
        // Temporary variable
        Lvalue::Temp(ref temp) => {
            // Find the index and type in the declaration
            let mut ty = data.2[temp.index()].ty.clone().to_string();
            match data.2[temp.index()].ty.sty {
                TypeVariants::TyTuple(ref t) => {
                    ty = t[0].to_string();
                },
                _ => { }
            }
            VariableMappingData{ name: "tmp".to_string() + temp.index().to_string().as_str(), var_type: ty }
        },
        // Local variable
        Lvalue::Var(ref var) => {
            // FIXME: fix comment
            // Find the name and type in the declaration
            VariableMappingData{ name: "var".to_string() + var.index().to_string().as_str(), var_type: data.3[var.index()].ty.clone().to_string() }
        },
        // The returned value
        Lvalue::ReturnPointer => {

            VariableMappingData{ name: "return".to_string(), var_type : data.4.clone() }
        },
        // (Most likely) a field of a tuple from a checked operation
        Lvalue::Projection(pro) => {
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

            // FIXME: Lots of intermediaries, should be condensed
            // Get the name of the variable being projected
            let mut lvalue_name = "".to_string();
            let mut lvalue_type = "".to_string();
            match pro.as_ref().base {
                // Argument
                Lvalue::Arg(ref arg) => {
                    // Return the name of the argument
                    lvalue_name = data.0[arg.index()].debug_name.as_str().to_string();
                    lvalue_type = data.0[arg.index()].ty.clone().to_string();
                },
                // Temporary variable
                Lvalue::Temp(ref temp) => {
                    // Return "temp<index>"
                    lvalue_name = "tmp".to_string() + temp.index().to_string().as_str();
                    lvalue_type = data.2[temp.index()].ty.clone().to_string();
                    match data.2[temp.index()].ty.sty {
                        TypeVariants::TyTuple(ref t) => {
                            lvalue_type = t[0].to_string();
                        },
                        _ => { unimplemented!() }
                    }
                },
                // Local variable
                Lvalue::Var(ref var) => {
                    // Return the name of the variable
                    let i = index.parse::<usize>().unwrap();
                    lvalue_name = "var".to_string() + var.index().to_string().as_str();

                    match data.3[var.index()].ty.sty {
                        TypeVariants::TyTuple(ref t) => {
                            lvalue_type = t[i].to_string();
                        },
                        _ => { unimplemented!() }
                    }
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

            // Get the index int from index_operand, then stick it in the VariableMappingData
            VariableMappingData{ name: lvalue_name + "." + index.as_str(), var_type: lvalue_type }
        },
        _=> {unimplemented!();}
    }
}

// Generates an appropriate Expression based on whatever is found as an operand, either a literal or some kind of variable/temp/field
pub fn gen_operand(operand: &Operand, data: &(Vec<&ArgDecl>, Vec<&BasicBlockData>, Vec<&TempDecl>, Vec<&VarDecl>, String)) -> Expression {
    match operand {
        // A variable/temp/field
        &Operand::Consume (ref l) => {
            Expression::VariableMapping( gen_lvalue(l.clone(), data) )
        },
        // A literal value
        &Operand::Constant (ref c) => {
            match c.literal {
                Literal::Item {ref def_id, ref substs} => { unimplemented!(); },
                Literal::Value {ref value} => {
                    match value {
                        &ConstVal::Integral(ref const_int) => {
                            match const_int {
                                &ConstInt::I8(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData { size: 8, value: i as i64 } )
                                },
                                &ConstInt::I16(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData { size: 16, value: i as i64 } )
                                },
                                &ConstInt::I32(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData { size: 32, value: i as i64 } )
                                },
                                &ConstInt::I64(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData { size: 64, value: i as i64 } )
                                },
                                &ConstInt::U8(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 8, value: u as u64 } )
                                },
                                &ConstInt::U16(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 16, value: u as u64 } )
                                },
                                &ConstInt::U32(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 32, value: u as u64 } )
                                },
                                &ConstInt::U64(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 64, value: u as u64 } )
                                },
                                _ => { unimplemented!(); }
                            }
                        },
                        _ => { unimplemented!(); },
                    }
                },
                Literal::Promoted {ref index} => { unimplemented!(); },
            }
        },
    }
}
