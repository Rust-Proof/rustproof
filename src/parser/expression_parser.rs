#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use expression::{Expression, BinaryExpressionData, UnaryExpressionData, VariableMappingData, UnsignedBitVectorData, SignedBitVectorData, BinaryOperator, UnaryOperator};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__E1 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::{Expression, BinaryExpressionData, UnaryExpressionData, VariableMappingData, UnsignedBitVectorData, SignedBitVectorData, BinaryOperator, UnaryOperator};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_E1<
        'input,
    >(
        input: &'input str,
    ) -> Result<Expression, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____E1((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        BOP1((usize, BinaryOperator, usize)),
        BOP2((usize, BinaryOperator, usize)),
        BOP3((usize, BinaryOperator, usize)),
        BOP4((usize, BinaryOperator, usize)),
        BOP5((usize, BinaryOperator, usize)),
        BOP6((usize, BinaryOperator, usize)),
        BOP7((usize, BinaryOperator, usize)),
        BOP8((usize, BinaryOperator, usize)),
        E1((usize, Expression, usize)),
        E10((usize, Expression, usize)),
        E2((usize, Expression, usize)),
        E3((usize, Expression, usize)),
        E4((usize, Expression, usize)),
        E5((usize, Expression, usize)),
        E6((usize, Expression, usize)),
        E7((usize, Expression, usize)),
        E8((usize, Expression, usize)),
        E9((usize, Expression, usize)),
        IDENTIFIER((usize, String, usize)),
        INT__BOUNDS((usize, Expression, usize)),
        TYPE((usize, String, usize)),
        UOP((usize, UnaryOperator, usize)),
        ____E1((usize, Expression, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym0));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym0));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym0));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym0));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym0));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym0));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym0));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym0));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym0));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym0));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym0));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym0));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym0));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym0));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym0));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym0));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym0));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym0));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym0));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym0));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym0));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym0));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym0));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym0));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym0));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E1(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E10(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E2(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state41(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state42(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state43(input, __tokens, __sym1));
            }
            Some((__loc1, (21, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state44(input, __tokens, __sym1));
            }
            Some((__loc1, (23, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state45(input, __tokens, __sym1));
            }
            Some((__loc1, (24, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state46(input, __tokens, __sym1));
            }
            Some((__loc1, (54, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state47(input, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____E1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state40(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __nt = __Nonterminal::E9((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym1));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym1));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state51(input, __tokens, __sym1));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state52(input, __tokens, __sym1));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state53(input, __tokens, __sym1));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(input, __tokens, __sym1));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __nt = __Nonterminal::E1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state48(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (53, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state56(input, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __nt = __Nonterminal::E2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state55(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (25, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state58(input, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __nt = __Nonterminal::E3((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP4(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state57(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state60(input, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __nt = __Nonterminal::E4((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP5(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state59(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym1));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(input, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::E5((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP6(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state61(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state66(input, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __nt = __Nonterminal::E6((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP7(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state64(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state68(input, __tokens, __sym1));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state69(input, __tokens, __sym1));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state70(input, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __nt = __Nonterminal::E7((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP8(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state67(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __nt = __Nonterminal::E8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state71(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, UnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym1));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym1));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym1));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym1));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym1));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym1));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym1));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym1));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym1));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym1));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym1));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym1));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym1));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym1));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym1));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym1));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym1));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym1));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym1));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym1));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym1));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state72(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37(input, __sym0);
                let __nt = __Nonterminal::UOP((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym1));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym1));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym1));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym1));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym1));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym1));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym1));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym1));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym1));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym1));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym1));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym1));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym1));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym1));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym1));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym1));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym1));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym1));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym1));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym1));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym1));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state73(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::E10(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state75(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state76(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state77(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state78(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36(input, __sym0);
                let __nt = __Nonterminal::UOP((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::UOP((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action74(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action78(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action76(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action80(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action73(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action77(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action82(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action86(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action83(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action87(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action84(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action88(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action85(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state107(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state108(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (11, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63(input, __sym0);
                let __nt = __Nonterminal::IDENTIFIER((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (11, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action62(input, __sym0);
                let __nt = __Nonterminal::IDENTIFIER((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E2(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state109(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40(input, __sym0);
                let __nt = __Nonterminal::BOP1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state48<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state110(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state49<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51(input, __sym0);
                let __nt = __Nonterminal::BOP2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state50<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46(input, __sym0);
                let __nt = __Nonterminal::BOP2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state51<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47(input, __sym0);
                let __nt = __Nonterminal::BOP2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state52<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50(input, __sym0);
                let __nt = __Nonterminal::BOP2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state53<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48(input, __sym0);
                let __nt = __Nonterminal::BOP2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state54<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49(input, __sym0);
                let __nt = __Nonterminal::BOP2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state55<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state111(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state56<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52(input, __sym0);
                let __nt = __Nonterminal::BOP3((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state57<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state112(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state58<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53(input, __sym0);
                let __nt = __Nonterminal::BOP4((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state59<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state113(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state60<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action54(input, __sym0);
                let __nt = __Nonterminal::BOP5((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state61<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state114(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state62<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55(input, __sym0);
                let __nt = __Nonterminal::BOP6((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state63<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56(input, __sym0);
                let __nt = __Nonterminal::BOP6((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state64<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state115(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state65<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57(input, __sym0);
                let __nt = __Nonterminal::BOP7((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state66<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action58(input, __sym0);
                let __nt = __Nonterminal::BOP7((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state67<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state116(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state68<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action61(input, __sym0);
                let __nt = __Nonterminal::BOP8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state69<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::BOP8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state70<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (35, _), _)) |
            Some((_, (36, _), _)) |
            Some((_, (38, _), _)) |
            Some((_, (39, _), _)) |
            Some((_, (40, _), _)) |
            Some((_, (42, _), _)) |
            Some((_, (43, _), _)) |
            Some((_, (45, _), _)) |
            Some((_, (46, _), _)) |
            Some((_, (48, _), _)) |
            Some((_, (49, _), _)) |
            Some((_, (51, _), _)) |
            Some((_, (52, _), _)) |
            Some((_, (55, _), _)) |
            Some((_, (56, _), _)) |
            Some((_, (57, _), _)) |
            Some((_, (58, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60(input, __sym0);
                let __nt = __Nonterminal::BOP8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state71<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (26, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state118(input, __tokens, __sym2));
            }
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state119(input, __tokens, __sym2));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state120(input, __tokens, __sym2));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state121(input, __tokens, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state122(input, __tokens, __sym2));
            }
            Some((__loc1, (41, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state123(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state124(input, __tokens, __sym2));
            }
            Some((__loc1, (47, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state125(input, __tokens, __sym2));
            }
            Some((__loc1, (50, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state126(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::TYPE(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state117(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state72<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, UnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17(input, __sym0, __sym1);
                let __nt = __Nonterminal::E9((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state73<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state41(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state128(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state42(input, __tokens, __sym2));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state43(input, __tokens, __sym2));
            }
            Some((__loc1, (21, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state44(input, __tokens, __sym2));
            }
            Some((__loc1, (23, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state45(input, __tokens, __sym2));
            }
            Some((__loc1, (24, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state46(input, __tokens, __sym2));
            }
            Some((__loc1, (54, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state47(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP1(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state127(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state74<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __nt = __Nonterminal::E9((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state75<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym1));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym1));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state51(input, __tokens, __sym1));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state52(input, __tokens, __sym1));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state53(input, __tokens, __sym1));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(input, __tokens, __sym1));
            }
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __nt = __Nonterminal::E1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state129(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state76<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (53, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state56(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __nt = __Nonterminal::E2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state130(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state77<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (25, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state58(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __nt = __Nonterminal::E3((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP4(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state131(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state78<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state60(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __nt = __Nonterminal::E4((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP5(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state132(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state79<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym1));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::E5((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP6(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state133(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state80<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state66(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __nt = __Nonterminal::E6((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP7(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state134(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state81<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state68(input, __tokens, __sym1));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state69(input, __tokens, __sym1));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state70(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __nt = __Nonterminal::E7((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP8(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state135(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state82<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __nt = __Nonterminal::E8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state83<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state136(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state84<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state85<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, UnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym1));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym1));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym1));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym1));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym1));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym1));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym1));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym1));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym1));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym1));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym1));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym1));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym1));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym1));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym1));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym1));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym1));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym1));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym1));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym1));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym1));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state137(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state86<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym1));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym1));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym1));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym1));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym1));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym1));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym1));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym1));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym1));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym1));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym1));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym1));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym1));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym1));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym1));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym1));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym1));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym1));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym1));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym1));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym1));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state138(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::E10(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state75(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state76(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state77(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state78(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state87<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state88<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action74(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state89<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action78(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state90<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state91<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state92<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action76(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state93<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action80(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state94<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action73(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state95<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action77(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state96<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state97<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action82(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state98<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action86(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state99<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action83(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state100<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action87(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state101<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action84(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state102<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action88(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state103<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state104<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action85(input, __sym0);
                let __nt = __Nonterminal::INT__BOUNDS((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state105<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state139(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state106<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state140(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state107<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state141(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state142(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state143(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state144(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state108<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state145(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state146(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state147(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state148(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (41, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state149(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state150(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (47, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state151(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (50, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state152(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state109<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym3));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym3));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state51(input, __tokens, __sym3));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state52(input, __tokens, __sym3));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state53(input, __tokens, __sym3));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(input, __tokens, __sym3));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP2(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state48(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state110<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (53, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state56(input, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP3(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state55(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state111<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (25, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state58(input, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E3((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP4(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state57(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state112<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state60(input, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E4((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP5(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state59(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state113<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym3));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(input, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E5((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP6(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state61(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state114<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(input, __tokens, __sym3));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state66(input, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E6((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP7(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state64(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state115<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state68(input, __tokens, __sym3));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state69(input, __tokens, __sym3));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state70(input, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E7((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP8(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state67(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state116<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state117<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action33(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state118<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state119<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state120<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action67(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state121<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state122<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state123<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state124<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action71(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state125<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action72(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state126<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state127<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E2(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state153(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state76(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state77(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state78(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state128<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Expression, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action35(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state129<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state154(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state77(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state78(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state130<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E4(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state155(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state78(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state131<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E5(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state156(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state132<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E6(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state157(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state133<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E7(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state158(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state134<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E8(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state159(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state82(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state135<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym2));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym2));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym2));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state94(input, __tokens, __sym2));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym2));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state97(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym2));
            }
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym2));
            }
            Some((__loc1, (46, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym2));
            }
            Some((__loc1, (48, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym2));
            }
            Some((__loc1, (49, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym2));
            }
            Some((__loc1, (51, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state103(input, __tokens, __sym2));
            }
            Some((__loc1, (52, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym2));
            }
            Some((__loc1, (55, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state105(input, __tokens, __sym2));
            }
            Some((__loc1, (56, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            Some((__loc1, (57, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym2));
            }
            Some((__loc1, (58, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E10(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state74(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::E9(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state160(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::IDENTIFIER(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::INT__BOUNDS(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state84(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state136<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (26, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state162(input, __tokens, __sym2));
            }
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state163(input, __tokens, __sym2));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state164(input, __tokens, __sym2));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state165(input, __tokens, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state166(input, __tokens, __sym2));
            }
            Some((__loc1, (41, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state167(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state168(input, __tokens, __sym2));
            }
            Some((__loc1, (47, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state169(input, __tokens, __sym2));
            }
            Some((__loc1, (50, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state170(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::TYPE(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state161(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state137<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, UnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17(input, __sym0, __sym1);
                let __nt = __Nonterminal::E9((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state138<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state41(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state171(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state42(input, __tokens, __sym2));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state43(input, __tokens, __sym2));
            }
            Some((__loc1, (21, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state44(input, __tokens, __sym2));
            }
            Some((__loc1, (23, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state45(input, __tokens, __sym2));
            }
            Some((__loc1, (24, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state46(input, __tokens, __sym2));
            }
            Some((__loc1, (54, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state47(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP1(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state127(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state139<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state172(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state173(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state174(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state175(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state140<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state176(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state177(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state178(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state179(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (41, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state180(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state181(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (47, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state182(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (50, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state183(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state141<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state142<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state143<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state144<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state145<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state146<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state147<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state148<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state149<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state150<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action31(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state151<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state152<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state153<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym3));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym3));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state51(input, __tokens, __sym3));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state52(input, __tokens, __sym3));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state53(input, __tokens, __sym3));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(input, __tokens, __sym3));
            }
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E1((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP2(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state129(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state154<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (53, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state56(input, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E2((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP3(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state130(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state155<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (25, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state58(input, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E3((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP4(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state131(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state156<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state60(input, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E4((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP5(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state132(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state157<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym3));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(input, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E5((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP6(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state133(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state158<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(input, __tokens, __sym3));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state66(input, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E6((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP7(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state134(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state159<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state68(input, __tokens, __sym3));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state69(input, __tokens, __sym3));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state70(input, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E7((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::BOP8(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state135(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state160<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Expression, usize)>,
        __sym1: &mut Option<(usize, BinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Expression, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E8((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state161<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action33(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state162<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state163<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state164<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action67(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state165<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state166<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state167<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state168<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action71(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state169<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action72(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state170<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::TYPE((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state171<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Expression, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action35(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state172<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state173<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state174<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state175<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state176<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state177<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state178<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state179<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state180<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state181<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action31(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state182<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state183<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) |
            Some((_, (14, _), _)) |
            Some((_, (15, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (53, _), _)) |
            Some((_, (54, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E10((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__E1::parse_E1;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        33 => /* '!' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        37 => /* '%' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        38 => /* '&' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((56, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        60 => /* '<' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_state = 13;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        65 => /* 'A' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        66 ... 68 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        70 ... 72 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        74 ... 77 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        80 ... 87 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        88 => /* 'X' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        89 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((25, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_state = 23;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        99 ... 101 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        103 ... 104 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        106 ... 115 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((53, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        38 => /* '&' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((55, __index + __ch.len_utf8()));
                            __current_state = 33;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((56, __index + __ch.len_utf8()));
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        60 => /* '<' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 77 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        79 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 80 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        81 => /* 'Q' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        82 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 76 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        77 => /* 'M' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        78 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 78 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 44;
                            continue;
                        }
                        80 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 81 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        83 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 78 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        80 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 47;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 47;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((58, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 47;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        49 => /* '1' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 50;
                            continue;
                        }
                        50 => /* '2' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        51 => /* '3' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 51;
                            continue;
                        }
                        52 ... 53 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        54 => /* '6' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 52;
                            continue;
                        }
                        55 => /* '7' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        56 => /* '8' */ {
                            __current_match = Some((37, __index + 1));
                            __current_state = 53;
                            continue;
                        }
                        57 => /* '9' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 54;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        49 => /* '1' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 55;
                            continue;
                        }
                        50 => /* '2' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        51 => /* '3' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 56;
                            continue;
                        }
                        52 ... 53 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        54 => /* '6' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        55 => /* '7' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        56 => /* '8' */ {
                            __current_match = Some((50, __index + 1));
                            __current_state = 58;
                            continue;
                        }
                        57 => /* '9' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        124 => /* '|' */ {
                            __current_match = Some((54, __index + 1));
                            __current_state = 59;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((55, __index + __ch.len_utf8()));
                            __current_state = 60;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((56, __index + __ch.len_utf8()));
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 67 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        68 => /* 'D' */ {
                            __current_match = Some((19, __index + 1));
                            __current_state = 61;
                            continue;
                        }
                        69 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 84 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 62;
                            continue;
                        }
                        86 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 79 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        80 => /* 'P' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 63;
                            continue;
                        }
                        81 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 83 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        84 => /* 'T' */ {
                            __current_match = Some((22, __index + 1));
                            __current_state = 64;
                            continue;
                        }
                        85 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 81 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((24, __index + 1));
                            __current_state = 65;
                            continue;
                        }
                        83 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 66;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 66;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((58, __index + 1));
                            __current_state = 66;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 66;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 67;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 68;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 53 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        54 => /* '6' */ {
                            __current_match = Some((28, __index + 1));
                            __current_state = 69;
                            continue;
                        }
                        55 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 49 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        50 => /* '2' */ {
                            __current_match = Some((31, __index + 1));
                            __current_state = 70;
                            continue;
                        }
                        51 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 51 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        52 => /* '4' */ {
                            __current_match = Some((34, __index + 1));
                            __current_state = 71;
                            continue;
                        }
                        53 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 72;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 73;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 53 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        54 => /* '6' */ {
                            __current_match = Some((41, __index + 1));
                            __current_state = 74;
                            continue;
                        }
                        55 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 49 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        50 => /* '2' */ {
                            __current_match = Some((44, __index + 1));
                            __current_state = 75;
                            continue;
                        }
                        51 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 51 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        52 => /* '4' */ {
                            __current_match = Some((47, __index + 1));
                            __current_state = 76;
                            continue;
                        }
                        53 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 77;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                59 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                60 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((55, __index + __ch.len_utf8()));
                            __current_state = 60;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                61 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                62 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 72 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 78;
                            continue;
                        }
                        74 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                63 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 75 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 79;
                            continue;
                        }
                        77 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                64 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                65 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                66 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 66;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 66;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((58, __index + 1));
                            __current_state = 66;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((58, __index + __ch.len_utf8()));
                            __current_state = 66;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                67 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((26, __index + 1));
                            __current_state = 80;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                68 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 81;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                69 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 82;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                70 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 83;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                71 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 84;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                72 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 85;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                73 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((40, __index + 1));
                            __current_state = 86;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                74 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 87;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                75 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 88;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                76 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 89;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                77 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 90;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                78 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 85 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        86 => /* 'V' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 91;
                            continue;
                        }
                        87 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                79 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 72 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 92;
                            continue;
                        }
                        74 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                80 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                81 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((27, __index + 1));
                            __current_state = 93;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                82 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 94;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                83 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 95;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                84 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 96;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                85 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 97;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                86 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                87 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 98;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                88 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 99;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                89 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 100;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                90 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 101;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                91 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                92 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 68 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 102;
                            continue;
                        }
                        70 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                93 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                94 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 103;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                95 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 104;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                96 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 105;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                97 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 106;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 107;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                98 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 108;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                99 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 109;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                100 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 110;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                101 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 111;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 112;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                102 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 82 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        83 => /* 'S' */ {
                            __current_match = Some((21, __index + 1));
                            __current_state = 113;
                            continue;
                        }
                        84 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                103 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 114;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 115;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                104 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 116;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 117;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                105 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 118;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 119;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                106 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((38, __index + 1));
                            __current_state = 120;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                107 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((39, __index + 1));
                            __current_state = 121;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                108 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 122;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 123;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                109 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 124;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 125;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                110 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 126;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 127;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                111 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((51, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                112 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((52, __index + 1));
                            __current_state = 129;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                113 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((57, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((57, __index + __ch.len_utf8()));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                114 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((29, __index + 1));
                            __current_state = 130;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                115 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((30, __index + 1));
                            __current_state = 131;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                116 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((32, __index + 1));
                            __current_state = 132;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                117 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((33, __index + 1));
                            __current_state = 133;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                118 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((35, __index + 1));
                            __current_state = 134;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                119 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((36, __index + 1));
                            __current_state = 135;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                120 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                121 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                122 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((42, __index + 1));
                            __current_state = 136;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                123 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((43, __index + 1));
                            __current_state = 137;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                124 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((45, __index + 1));
                            __current_state = 138;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                125 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((46, __index + 1));
                            __current_state = 139;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                126 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_match = Some((48, __index + 1));
                            __current_state = 140;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                127 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((49, __index + 1));
                            __current_state = 141;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                128 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                129 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                130 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                131 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                132 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                133 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                134 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                135 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                136 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                137 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                138 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                139 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                140 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                141 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, op, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::BinaryExpression( BinaryExpressionData { op: op, left: Box::new(left), right: Box::new(right) } )
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, op, _): (usize, UnaryOperator, usize),
    (_, e, _): (usize, Expression, usize),
) -> Expression
{
    Expression::UnaryExpression( UnaryExpressionData { op: op, e: Box::new(e) } )
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::BooleanLiteral(true)
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::BooleanLiteral(false)
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 8, value: i64::from_str(i).unwrap() } )
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 16, value: i64::from_str(i).unwrap() } )
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 32, value: i64::from_str(i).unwrap() } )
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 64, value: i64::from_str(i).unwrap() } )
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 8, value: i64::from_str(i).unwrap() } )
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 16, value: i64::from_str(i).unwrap() } )
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 32, value: i64::from_str(i).unwrap() } )
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 64, value: i64::from_str(i).unwrap() } )
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 8, value: u64::from_str(i).unwrap() } )
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 16, value: u64::from_str(i).unwrap() } )
}

pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 32, value: u64::from_str(i).unwrap() } )
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 64, value: u64::from_str(i).unwrap() } )
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, String, usize),
) -> Expression
{
    Expression::VariableMapping( VariableMappingData { name: n, var_type: t } )
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, ib, _): (usize, Expression, usize),
) -> Expression
{
    ib
}

pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Expression, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    e
}

pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnaryOperator
{
    UnaryOperator::Negation
}

pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnaryOperator
{
    UnaryOperator::BitwiseNot
}

pub fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnaryOperator
{
    UnaryOperator::Not
}

pub fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::And
}

pub fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Or
}

pub fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::And
}

pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Or
}

pub fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Xor
}

pub fn __action44<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Implication
}

pub fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BiImplication
}

pub fn __action46<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::LessThan
}

pub fn __action47<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::LessThanOrEqual
}

pub fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::GreaterThan
}

pub fn __action49<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::GreaterThanOrEqual
}

pub fn __action50<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Equal
}

pub fn __action51<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::NotEqual
}

pub fn __action52<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BitwiseOr
}

pub fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BitwiseXor
}

pub fn __action54<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BitwiseAnd
}

pub fn __action55<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BitwiseLeftShift
}

pub fn __action56<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BitwiseRightShift
}

pub fn __action57<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Addition
}

pub fn __action58<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Subtraction
}

pub fn __action59<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Multiplication
}

pub fn __action60<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Division
}

pub fn __action61<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Modulo
}

pub fn __action62<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
) -> String
{
    i.to_string()
}

pub fn __action63<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
) -> String
{
    i.to_string()
}

pub fn __action64<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "bool".to_string()
}

pub fn __action65<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "i8".to_string()
}

pub fn __action66<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "i16".to_string()
}

pub fn __action67<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "i32".to_string()
}

pub fn __action68<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "i64".to_string()
}

pub fn __action69<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "u8".to_string()
}

pub fn __action70<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "u16".to_string()
}

pub fn __action71<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "u32".to_string()
}

pub fn __action72<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "u64".to_string()
}

pub fn __action73<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 8, value: i8::max_value() as i64 } )
}

pub fn __action74<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 16, value: i16::max_value() as i64 } )
}

pub fn __action75<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 32, value: i32::max_value() as i64 } )
}

pub fn __action76<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 64, value: i64::max_value() as i64 } )
}

pub fn __action77<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 8, value: i8::min_value() as i64 } )
}

pub fn __action78<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 16, value: i16::min_value() as i64 } )
}

pub fn __action79<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 32, value: i32::min_value() as i64 } )
}

pub fn __action80<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::SignedBitVector( SignedBitVectorData { size: 64, value: i64::min_value() as i64 } )
}

pub fn __action81<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 8, value: u8::max_value() as u64 } )
}

pub fn __action82<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 16, value: u16::max_value() as u64 } )
}

pub fn __action83<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 32, value: u32::max_value() as u64 } )
}

pub fn __action84<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 64, value: u64::max_value() as u64 } )
}

pub fn __action85<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 8, value: u8::min_value() as u64 } )
}

pub fn __action86<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 16, value: u16::min_value() as u64 } )
}

pub fn __action87<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 32, value: u32::min_value() as u64 } )
}

pub fn __action88<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::UnsignedBitVector( UnsignedBitVectorData { size: 64, value: u64::min_value() as u64 } )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
