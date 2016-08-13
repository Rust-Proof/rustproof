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

use super::DEBUG;
use super::MirData;
use std::process;
use expression::*;
use rustc::mir::repr::*;
use rustc::middle::const_val::ConstVal;
use rustc_const_math::ConstInt;
use rustc_data_structures::indexed_vec::Idx;
use rustc::ty::{TypeVariants};
use std::rt::begin_panic_fmt;

use errors::{ColorConfig, Handler};
use syntax::codemap::CodeMap;
use std::rc::Rc;

mod overflow;

/// Computes the weakest precondition for a given post-condition and series of statments over one or more BasicBlocks from MIR.
///
/// # Arguments:
/// * `index` - Index of the ArgDecl,TempDecl, OR VarDecl within the respective Vec<>
/// * `data` - Contains the BasicBlockData and all of the ArgDecls, TempDecls, and VarDecls from the MIR pass
/// * `post_expr` - Contains the post-condition in expression form
///
/// # Return Value:
/// * returns the weakest precondtion in the form of an expression within an Option. This is sent to FIXME
///
/// # Remarks:
/// * This is the main generator for the weakest precondtion and is called to generate the weakest precondition
///
/// # Debug:
/// * to find debugging statments, grep for "DEBUG PURPOSE:"
///
pub fn gen(index: usize, data: &mut MirData, post_expr: &Option<Expression>) -> Option<Expression> {
    // FIXME: Debug should not be a const; it must be user-facing
    // DEBUG PURPOSE:
    // Shows the current BasicBlock index and the BasicBLockData associated with that index
    if DEBUG { println!("Examining bb{:?}\n{:#?}\n", index, data.block_data[index]); }
    let mut wp: Option<Expression> = None;

    // Parse terminator data
    let terminator = data.block_data[index].terminator.clone().unwrap().kind;
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => {
            // Retrieve the weakest precondition from the following block
            wp = gen(target.index(), data, post_expr);
        },
        TerminatorKind::Return => {
            // Return the post condition to the preceeding block
            return post_expr.clone();
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
            //FIXME: Is this unimplemented!() needed?
            unimplemented!();
        },
        // TerminatorKind::If requires special generation of wp.
        // wp(if c x else y) = (c -> x) AND ((NOT c) -> y)
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
                Operand::Consume(c) => { Expression::VariableMapping(gen_lvalue(c, data)) },
            };
            // Negate the conditional expression
            let not_condition = Expression::UnaryExpression(UnaryExpressionData {
                op: UnaryOperator::Not,
                e: Box::new(condition.clone())
            });
            // wp(If c x else y) = (c -> x) AND ((NOT c) -> y)
            wp = Some(Expression::BinaryExpression(BinaryExpressionData {
                op: BinaryOperator::And,
                left: Box::new(Expression::BinaryExpression(BinaryExpressionData {
                    op: BinaryOperator::Implication,
                    left: Box::new(condition.clone()),
                    right: Box::new(wp_if.unwrap())
                })),
                right: Box::new(Expression::BinaryExpression(BinaryExpressionData {
                    op: BinaryOperator::Implication,
                    left: Box::new(not_condition.clone()),
                    right: Box::new(wp_else.unwrap())
                }))
            }));
        },
        // Unimplemented TerminatorKinds
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::Switch{discr, adt_def, targets} => unimplemented!(),
        TerminatorKind::SwitchInt{discr, switch_ty, values, targets} => unimplemented!(),
    }

    // Examine statements in reverse order
    let mut stmts = data.block_data[index].statements.clone();
    stmts.reverse();
    // DEBUG PURPOSE:
    // displays the current BasicBlock index
    if DEBUG { println!("bb{:?}", index);}
    for stmt in stmts {
        // Modify the weakest precondition based on the statement
        wp = gen_stmt(wp.unwrap(), stmt, data);
    }
    // DEBUG PURPOSE:
    // shows the result to be returned to the proceeding block
    if DEBUG { println!("wp returned as\t{:?}\n", wp.clone().unwrap()); }

    // Return the weakest precondition to the preceeding block, or to control
    wp
}


/// Returns the type of an operand as a String
///
/// # Arguments:
/// * `oeprand` - Index of the ArgDecl,TempDecl, OR VarDecl within the respective Vec<>
/// * `data` - Contains the BasicBlockData and all of the ArgDecls, TempDecls, and VarDecls from the MIR pass
///
/// # Remarks:
/// * This is the main generator for the weakest precondtion and is called to generate the weakest precondition
/// * May be depercated in the near future
///
fn gen_ty(operand: &Operand, data: &mut MirData) -> String {
    match operand.clone() {
        Operand::Constant(ref constant) => { constant.ty.to_string() },
        Operand::Consume(ref lvalue) => {
            match lvalue {
                // Function argument
                &Lvalue::Arg(ref arg) => {
                    data.arg_data[arg.index()].ty.to_string()
                },
                // Temporary variable
                &Lvalue::Temp(ref temp) => {
                    data.temp_data[temp.index()].ty.to_string()
                },
                // Local variable
                &Lvalue::Var(ref var) => {
                    data.var_data[var.index()].ty.to_string()
                },
                _ => {
                    unimplemented!();
                }
            }
        }
    }
}




/// Generates a version of wp "And"ed together with a conditional expression that mimics a check to ensure division by 0 does not occur.
///
/// # Arguments:
/// * `wp` - The current weakest precondition that the "div by 0" is to be "And"ed to
/// * `exp` - The expression to check to make sure it is not divided by 0
///
/// # Return Value:
/// * Returns the modified weakest precondition with "div by 0" Expression "And"ed
///
/// # Remarks:
/// * Current supported ConstInt: I8, I16, I32, I64, U8, U16, U32, U64
///
pub fn add_zero_check(wp: &Expression, exp: &Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        // And weakest precondtion and ovflow check
        op: BinaryOperator::And,
        left: Box::new(wp.clone()),
        right: Box::new(Expression::BinaryExpression( BinaryExpressionData{
            op: BinaryOperator::NotEqual,
            // left side of new Expression to check for divided by 0
            left: Box::new(exp.clone()),
            // Need to set appropriate type with value of 0
            right: Box::new(Expression::SignedBitVector( SignedBitVectorData {
                // The bit-vector size of the given type
                size: match determine_evaluation_type(exp).as_str() {
                    "i8" => { 8 },
                    "i16" => { 16 },
                    "i32" => { 32 },
                    "i64" => { 64 },
                    "u8" => { 8 },
                    "u16" => { 16 },
                    "u32" => { 32 },
                    "u64" => { 64 },
                    _ => { rp_error!("Unimplemented checkeddAdd right-hand operand type") }
                },
                value: 0
            }))
        }))
    })
}


/// Returns a (possibly) modified weakest precondition based on the content of a statement
///
/// # Arguments:
/// * `wp` - The current weakest precondition
/// * `stmt` - The Statment to be processed into wp
/// * `data` - Contains the BasicBlockData and all of the ArgDecls, TempDecls, and VarDecls from the MIR pass
///
/// # Return Value:
/// * Returns the modified weakest precondition with underflow check
///
/// # Remarks:
///
/// # Debug:
/// * to find debugging statments, grep for "DEBUG PURPOSE:
///
fn gen_stmt(mut wp: Expression, stmt: Statement, data: &mut MirData) -> Option<Expression>  {
    // DEBUG PURPOSE:
    // uncomment out this line for debug purposes. It will show the current statement it is processing
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
            let lvalue: Expression = gen_expression(&loperand, data);
            let rvalue: Expression = gen_expression(&roperand, data);
            let op: BinaryOperator = match binop {
                &BinOp::Add => {
                    // Add the overflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    BinaryOperator::Addition
                },
                &BinOp::Sub => {
                    // Add the overflow and underflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    BinaryOperator::Subtraction
                },
                &BinOp::Mul => {
                    // Add the overflow and underflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    BinaryOperator::Multiplication
                },
                &BinOp::Div => {
                    // Add the overflow and underflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    // Add the division by 0 expression check
                    wp = add_zero_check(&wp, &rvalue);
                    BinaryOperator::Division
                },
                &BinOp::Rem => {
                    // Add the division by 0 expression check
                    wp = add_zero_check(&wp, &rvalue);
                    BinaryOperator::Modulo
                },
                &BinOp::Shl => { BinaryOperator::BitwiseLeftShift },
                &BinOp::Shr => { BinaryOperator::BitwiseRightShift },
                _ => { rp_error!("Unsupported checked binary operation!"); }
            };

            var.name = var.name + ".0";

            // add the new BinaryExpressionData on to the expression: Vec<>
            expression.push(Expression::BinaryExpression( BinaryExpressionData {
                op: op,
                left: Box::new(lvalue),
                right: Box::new(rvalue)
            } ));
        },

        Rvalue::BinaryOp(ref binop, ref lval, ref rval) => {
            let lvalue: Expression = gen_expression(&lval, data);
            let rvalue: Expression = gen_expression(&rval, data);
            let op: BinaryOperator = match binop {
                &BinOp::Add => {
                    // Add the overflow expression check
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    BinaryOperator::Addition
                },
                &BinOp::Sub => {
                    // Add the overflow and underflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    BinaryOperator::Subtraction
                },
                &BinOp::Mul => {
                    // Add the overflow and underflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    BinaryOperator::Multiplication
                },
                &BinOp::Div => {
                    // Add the overflow and underflow expression checks
                    wp = overflow::overflow_check(&wp, &var, binop, &lvalue, &rvalue);
                    // add the division by 0 expression check
                    wp = add_zero_check(&wp, &rvalue);
                    BinaryOperator::Division
                },
                &BinOp::Rem => {
                    // Add the division by 0 expression check
                    wp = add_zero_check(&wp, &rvalue);
                    BinaryOperator::Modulo
                },
                &BinOp::BitOr => { BinaryOperator::BitwiseOr },
                &BinOp::BitAnd => { BinaryOperator::BitwiseAnd },
                &BinOp::BitXor => { BinaryOperator::BitwiseXor },
                &BinOp::Shl => { BinaryOperator::BitwiseLeftShift },
                &BinOp::Shr => { BinaryOperator::BitwiseRightShift },
                &BinOp::Lt => { BinaryOperator::LessThan },
                &BinOp::Le => { BinaryOperator::LessThanOrEqual },
                &BinOp::Gt => { BinaryOperator::GreaterThan },
                &BinOp::Ge => { BinaryOperator::GreaterThanOrEqual },
                &BinOp::Eq => { BinaryOperator::Equal },
                &BinOp::Ne => { BinaryOperator::NotEqual },
            };
            // adds the new BinaryExpression to the expression: Vec<>
            expression.push(Expression::BinaryExpression( BinaryExpressionData {
                op: op,
                left: Box::new(lvalue),
                right: Box::new(rvalue)
            } ));
        },
        // Generates Rvalue to a UnaryOp
        Rvalue::UnaryOp(ref unop, ref val) => {
            let exp: Expression = gen_expression(&val, data);
            let op: UnaryOperator = match unop {
                &UnOp::Not => {
                    if determine_evaluation_type(&exp) == "bool" {
                        UnaryOperator::Not
                    } else {
                        UnaryOperator::BitwiseNot
                    }
                },
                &UnOp::Neg => { UnaryOperator::Negation },
            };
            // push the ne new exp onto the expression: Vec<>
            expression.push(Expression::UnaryExpression( UnaryExpressionData {
                op: op,
                e: Box::new(exp)
            } ));
        },
        //  FIXME: need def
        Rvalue::Use(ref operand) => {
            expression.push(gen_expression(operand, data));
        },
        //  FIXME: need def
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
                // &AggregateKind::Vec => { unimplemented!() },
                _ => { rp_error!("Unsupported aggregate: only tuples are supported"); }
            }
        },
        // FIXME: need def
        Rvalue::Cast(ref cast_kind, ref cast_operand, ref cast_ty) => {
            expression.push(Expression::VariableMapping(var.clone()));
        },
        // FIXME: need def
        Rvalue::Ref(ref ref_region, ref ref_borrow_kind, ref ref_lvalue) => {
            expression.push(Expression::VariableMapping(var.clone()));
        },
        // Unimplemented Rvalues
        Rvalue::Box(..) => { unimplemented!(); },
        Rvalue::Len(..) => { unimplemented!(); },
        _ => { unimplemented!(); }
    };

    // Replace any appearance of var in the weakest precondition with the expression
    for i in 0..expression.len() {
        substitute_variable_with_expression( &mut wp, &var, &expression[i] );
    }
    // DEBUG PURPOSE:
    // prints out the new weakest precondtion expression
    if DEBUG { println!("new expression\t\t{:?}\n--------------------------------", wp.clone());}
    return Some(wp);
}


/// Generates an appropriate variable mapping based on whatever variable, temp, or field is found
///
/// # Arguments:
/// * `lvalue` - The Lvalue to be generated into a VariableMapping
/// * `data` - Contains the BasicBlockData and all of the ArgDecls, TempDecls, and VarDecls from the MIR pass
///
/// # Return Value:
/// * Returns a VariableMappingData that is built from the data and lvalue
///
/// # Remarks:
///
///
fn gen_lvalue(lvalue: Lvalue, data: &mut MirData) -> VariableMappingData {
    match lvalue {
        // Function argument
        Lvalue::Arg(ref arg) => {
            // Find the name and type in the declaration
            VariableMappingData{name: data.arg_data[arg.index()].debug_name.as_str().to_string(),
                                var_type: data.arg_data[arg.index()].ty.clone().to_string()
            }
        },
        // Temporary variable
        Lvalue::Temp(ref temp) => {
            // Find the index and type in the declaration
            let mut ty = data.temp_data[temp.index()].ty.clone().to_string();
            match data.temp_data[temp.index()].ty.sty {
                TypeVariants::TyTuple(ref t) => { ty = t[0].to_string(); },
                _ => { }
            }
            VariableMappingData{name: "tmp".to_string() + temp.index().to_string().as_str(),
                                var_type: ty
            }
        },
        // Local variable
        Lvalue::Var(ref var) => {
            // FIXME: fix comment
            // Find the name and type in the declaration
            VariableMappingData{name: "var".to_string() + var.index().to_string().as_str(),
                                var_type: data.var_data[var.index()].ty.clone().to_string() }
        },
        // The returned value
        Lvalue::ReturnPointer => {
            VariableMappingData{name: "return".to_string(), var_type : data.func_return_type.clone() }
        },
        // (Most likely) a field of a tuple from a checked operation
        Lvalue::Projection(pro) => {
            // FIXME: Lots of intermediaries, should be condensed
            // Get the index
            let index: String = match pro.as_ref().elem.clone() {
                ProjectionElem::Index(ref o) => { unimplemented!(); },
                ProjectionElem::Field(ref field, ref ty) => { (field.index() as i32).to_string() }
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
                    lvalue_name = data.arg_data[arg.index()].debug_name.as_str().to_string();
                    lvalue_type = data.arg_data[arg.index()].ty.clone().to_string();
                },
                // Temporary variable
                Lvalue::Temp(ref temp) => {
                    // Return "temp<index>"
                    lvalue_name = "tmp".to_string() + temp.index().to_string().as_str();
                    lvalue_type = data.temp_data[temp.index()].ty.clone().to_string();
                    match data.temp_data[temp.index()].ty.sty {
                        TypeVariants::TyTuple(ref t) => { lvalue_type = t[0].to_string(); },
                        _ => { unimplemented!() },
                    }
                },
                // Local variable
                Lvalue::Var(ref var) => {
                    // Return the name of the variable
                    let i = index.parse::<usize>().unwrap();
                    lvalue_name = "var".to_string() + var.index().to_string().as_str();
                    match data.var_data[var.index()].ty.sty {
                        TypeVariants::TyTuple(ref t) => {
                            lvalue_type = t[i].to_string();
                        },
                        _ => { unimplemented!() },
                    }
                },
                // Unimplemented Lvalue
                Lvalue::ReturnPointer => { unimplemented!(); },
                Lvalue::Static(ref stat) => { unimplemented!(); },
                // Multiply-nested projection
                Lvalue::Projection(ref proj) => { unimplemented!(); },
            };

            // Get the index
            let index: String = match pro.as_ref().elem.clone() {

                ProjectionElem::Field(ref field, ref ty) => { (field.index() as i32).to_string() },
                ProjectionElem::Index(ref o) => { unimplemented!(); },
                _ => { unimplemented!(); }
            };

            // Get the index int from index_operand, then stick it in the VariableMappingData
            VariableMappingData{ name: lvalue_name + "." + index.as_str(), var_type: lvalue_type }
        },
        _=> { unimplemented!(); }
    }
}


/// Generates an appropriate Expression based on whatever is found as an operand, either a literal or some kind of variable/temp/field
///
/// # Arguments:
/// * `operand` - The Operand to generate a new expression from.
/// * `data` - Contains the BasicBlockData and all of the ArgDecls, TempDecls, and VarDecls from the MIR pass
///
/// # Return Value:
/// * Returns a new expression generated from an operand
///
/// # Remarks:
/// * Current Supported ConstVal: Bool, Integral
/// * Current supported Integral: I8, I16, I32, I64, U8, U16, U32, U64
///
fn gen_expression(operand: &Operand, data: &mut MirData) -> Expression {
    match operand {
        // A variable/temp/field
        &Operand::Consume (ref l) => {
            Expression::VariableMapping( gen_lvalue(l.clone(), data) )
        },
        // A literal value
        &Operand::Constant (ref c) => {
            match c.literal {
                Literal::Value {ref value} => {
                    match value {
                        &ConstVal::Bool(ref const_bool) => {
                            Expression::BooleanLiteral(*const_bool)
                        }
                        &ConstVal::Integral(ref const_int) => {
                            // get the correct time of the Integral
                            match const_int {
                                &ConstInt::I8(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: 8,
                                        value: i as i64
                                    } )
                                },
                                &ConstInt::I16(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: 16,
                                        value: i as i64
                                    } )
                                },
                                &ConstInt::I32(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: 32,
                                        value: i as i64
                                    } )
                                },
                                &ConstInt::I64(i) => {
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: 64,
                                        value: i as i64
                                    } )
                                },
                                &ConstInt::U8(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData {
                                        size: 8,
                                        value: u as u64
                                    } )
                                },
                                &ConstInt::U16(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData {
                                        size: 16,
                                        value: u as u64
                                    } )
                                },
                                &ConstInt::U32(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData {
                                        size: 32,
                                        value: u as u64
                                    } )
                                },
                                &ConstInt::U64(u) => {
                                    Expression::UnsignedBitVector( UnsignedBitVectorData {
                                        size: 64,
                                        value: u as u64
                                    } )
                                },
                                _ => { unimplemented!(); }
                            }
                        },
                        _ => { unimplemented!(); },
                    }
                },
                // unimplemented Literals
                Literal::Item {ref def_id, ref substs} => { unimplemented!(); },
                Literal::Promoted {ref index} => { unimplemented!(); },
            }
        },
    }
}
