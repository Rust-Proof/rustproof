#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use expression::Predicate;
use expression::Term;
use expression::NotData;
use expression::AndData;
use expression::OrData;
use expression::ImpliesData;
use expression::IntegerComparisonData;
use expression::UnsignedBitVectorData;
use expression::VariableMappingData;
use expression::UnaryExpressionData;
use expression::BinaryExpressionData;
use expression::IntegerComparisonOperator;
use expression::IntegerUnaryOperator;
use expression::IntegerBinaryOperator;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__INT_BOP {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::Predicate;
    use expression::Term;
    use expression::NotData;
    use expression::AndData;
    use expression::OrData;
    use expression::ImpliesData;
    use expression::IntegerComparisonData;
    use expression::UnsignedBitVectorData;
    use expression::VariableMappingData;
    use expression::UnaryExpressionData;
    use expression::BinaryExpressionData;
    use expression::IntegerComparisonOperator;
    use expression::IntegerUnaryOperator;
    use expression::IntegerBinaryOperator;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_INT_BOP<
        'input,
    >(
        input: &'input str,
    ) -> Result<IntegerBinaryOperator, __ParseError<usize,(usize, &'input str),()>>
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
            (None, __Nonterminal::____INT__BOP((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        INT__BOP((usize, IntegerBinaryOperator, usize)),
        INT__COMP((usize, IntegerComparisonOperator, usize)),
        INT__UOP((usize, IntegerUnaryOperator, usize)),
        T1((usize, Term, usize)),
        T2((usize, Term, usize)),
        T3((usize, Term, usize)),
        ____INT__BOP((usize, IntegerBinaryOperator, usize)),
        ____INT__COMP((usize, IntegerComparisonOperator, usize)),
        ____INT__UOP((usize, IntegerUnaryOperator, usize)),
        ____T1((usize, Term, usize)),
        ____T2((usize, Term, usize)),
        ____T3((usize, Term, usize)),
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
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state2(input, __tokens, __sym0));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym0));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym0));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym0));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym0));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym0));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym0));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym0));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym0));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(input, __tokens, __sym0));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
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
        __sym0: &mut Option<(usize, IntegerBinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __nt = __Nonterminal::____INT__BOP((
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

    pub fn __state2<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state4<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state5<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state6<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state7<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state8<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state9<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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

    pub fn __state10<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
pub use self::__parse__INT_BOP::parse_INT_BOP;

mod __parse__INT_COMP {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::Predicate;
    use expression::Term;
    use expression::NotData;
    use expression::AndData;
    use expression::OrData;
    use expression::ImpliesData;
    use expression::IntegerComparisonData;
    use expression::UnsignedBitVectorData;
    use expression::VariableMappingData;
    use expression::UnaryExpressionData;
    use expression::BinaryExpressionData;
    use expression::IntegerComparisonOperator;
    use expression::IntegerUnaryOperator;
    use expression::IntegerBinaryOperator;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_INT_COMP<
        'input,
    >(
        input: &'input str,
    ) -> Result<IntegerComparisonOperator, __ParseError<usize,(usize, &'input str),()>>
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
            (None, __Nonterminal::____INT__COMP((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        INT__BOP((usize, IntegerBinaryOperator, usize)),
        INT__COMP((usize, IntegerComparisonOperator, usize)),
        INT__UOP((usize, IntegerUnaryOperator, usize)),
        T1((usize, Term, usize)),
        T2((usize, Term, usize)),
        T3((usize, Term, usize)),
        ____INT__BOP((usize, IntegerBinaryOperator, usize)),
        ____INT__COMP((usize, IntegerComparisonOperator, usize)),
        ____INT__UOP((usize, IntegerUnaryOperator, usize)),
        ____T1((usize, Term, usize)),
        ____T2((usize, Term, usize)),
        ____T3((usize, Term, usize)),
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
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state2(input, __tokens, __sym0));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym0));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym0));
            }
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym0));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym0));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym0));
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
                __Nonterminal::INT__COMP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
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
        __sym0: &mut Option<(usize, IntegerComparisonOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __nt = __Nonterminal::____INT__COMP((
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

    pub fn __state2<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __nt = __Nonterminal::INT__COMP((
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __nt = __Nonterminal::INT__COMP((
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

    pub fn __state4<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __nt = __Nonterminal::INT__COMP((
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

    pub fn __state5<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __nt = __Nonterminal::INT__COMP((
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

    pub fn __state6<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __nt = __Nonterminal::INT__COMP((
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

    pub fn __state7<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __nt = __Nonterminal::INT__COMP((
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
pub use self::__parse__INT_COMP::parse_INT_COMP;

mod __parse__INT_UOP {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::Predicate;
    use expression::Term;
    use expression::NotData;
    use expression::AndData;
    use expression::OrData;
    use expression::ImpliesData;
    use expression::IntegerComparisonData;
    use expression::UnsignedBitVectorData;
    use expression::VariableMappingData;
    use expression::UnaryExpressionData;
    use expression::BinaryExpressionData;
    use expression::IntegerComparisonOperator;
    use expression::IntegerUnaryOperator;
    use expression::IntegerBinaryOperator;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_INT_UOP<
        'input,
    >(
        input: &'input str,
    ) -> Result<IntegerUnaryOperator, __ParseError<usize,(usize, &'input str),()>>
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
            (None, __Nonterminal::____INT__UOP((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        INT__BOP((usize, IntegerBinaryOperator, usize)),
        INT__COMP((usize, IntegerComparisonOperator, usize)),
        INT__UOP((usize, IntegerUnaryOperator, usize)),
        T1((usize, Term, usize)),
        T2((usize, Term, usize)),
        T3((usize, Term, usize)),
        ____INT__BOP((usize, IntegerBinaryOperator, usize)),
        ____INT__COMP((usize, IntegerComparisonOperator, usize)),
        ____INT__UOP((usize, IntegerUnaryOperator, usize)),
        ____T1((usize, Term, usize)),
        ____T2((usize, Term, usize)),
        ____T3((usize, Term, usize)),
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
                __result = try!(__state2(input, __tokens, __sym0));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym0));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
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
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __nt = __Nonterminal::____INT__UOP((
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

    pub fn __state2<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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
pub use self::__parse__INT_UOP::parse_INT_UOP;

mod __parse__T1 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::Predicate;
    use expression::Term;
    use expression::NotData;
    use expression::AndData;
    use expression::OrData;
    use expression::ImpliesData;
    use expression::IntegerComparisonData;
    use expression::UnsignedBitVectorData;
    use expression::VariableMappingData;
    use expression::UnaryExpressionData;
    use expression::BinaryExpressionData;
    use expression::IntegerComparisonOperator;
    use expression::IntegerUnaryOperator;
    use expression::IntegerBinaryOperator;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_T1<
        'input,
    >(
        input: &'input str,
    ) -> Result<Term, __ParseError<usize,(usize, &'input str),()>>
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
            (None, __Nonterminal::____T1((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        INT__BOP((usize, IntegerBinaryOperator, usize)),
        INT__COMP((usize, IntegerComparisonOperator, usize)),
        INT__UOP((usize, IntegerUnaryOperator, usize)),
        T1((usize, Term, usize)),
        T2((usize, Term, usize)),
        T3((usize, Term, usize)),
        ____INT__BOP((usize, IntegerBinaryOperator, usize)),
        ____INT__COMP((usize, IntegerComparisonOperator, usize)),
        ____INT__UOP((usize, IntegerUnaryOperator, usize)),
        ____T1((usize, Term, usize)),
        ____T2((usize, Term, usize)),
        ____T3((usize, Term, usize)),
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
                __result = try!(__state5(input, __tokens, __sym0));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym0));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym0));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym0));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym0));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym0));
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
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym1));
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
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym0, __sym1));
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym1));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym1));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym1));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym1));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym1));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym1));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____T1((
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __nt = __Nonterminal::T1((
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

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __nt = __Nonterminal::T2((
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

    pub fn __state5<
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
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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

    pub fn __state6<
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
                __result = try!(__state5(input, __tokens, __sym1));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym1));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __tokens, __lookahead, __sym1));
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
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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

    pub fn __state8<
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state9<
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1);
                let __nt = __Nonterminal::T2((
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
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym2));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym2));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym2));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym2));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym1));
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
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym2));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __nt = __Nonterminal::T1((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __nt = __Nonterminal::T2((
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
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym1));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym1));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state33(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::T3((
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __nt = __Nonterminal::T3((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T1((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1);
                let __nt = __Nonterminal::T2((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym2));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym2));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T3((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym2));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym2));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T1((
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
        __sym1: &mut Option<(usize, Term, usize)>,
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T3((
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
pub use self::__parse__T1::parse_T1;

mod __parse__T2 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::Predicate;
    use expression::Term;
    use expression::NotData;
    use expression::AndData;
    use expression::OrData;
    use expression::ImpliesData;
    use expression::IntegerComparisonData;
    use expression::UnsignedBitVectorData;
    use expression::VariableMappingData;
    use expression::UnaryExpressionData;
    use expression::BinaryExpressionData;
    use expression::IntegerComparisonOperator;
    use expression::IntegerUnaryOperator;
    use expression::IntegerBinaryOperator;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_T2<
        'input,
    >(
        input: &'input str,
    ) -> Result<Term, __ParseError<usize,(usize, &'input str),()>>
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
            (None, __Nonterminal::____T2((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        INT__BOP((usize, IntegerBinaryOperator, usize)),
        INT__COMP((usize, IntegerComparisonOperator, usize)),
        INT__UOP((usize, IntegerUnaryOperator, usize)),
        T1((usize, Term, usize)),
        T2((usize, Term, usize)),
        T3((usize, Term, usize)),
        ____INT__BOP((usize, IntegerBinaryOperator, usize)),
        ____INT__COMP((usize, IntegerComparisonOperator, usize)),
        ____INT__UOP((usize, IntegerUnaryOperator, usize)),
        ____T1((usize, Term, usize)),
        ____T2((usize, Term, usize)),
        ____T3((usize, Term, usize)),
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
                __result = try!(__state4(input, __tokens, __sym0));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym0));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym0));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym0));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym0));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
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
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym1));
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
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym0, __sym1));
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __nt = __Nonterminal::____T2((
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __nt = __Nonterminal::T2((
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

    pub fn __state4<
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
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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

    pub fn __state5<
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
                __result = try!(__state4(input, __tokens, __sym1));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T3(__nt) => {
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

    pub fn __state6<
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
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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

    pub fn __state7<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state8<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1);
                let __nt = __Nonterminal::T2((
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

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
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
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __nt = __Nonterminal::T1((
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __nt = __Nonterminal::T2((
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
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym1));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T3(__nt) => {
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::T3((
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __nt = __Nonterminal::T3((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1);
                let __nt = __Nonterminal::T2((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym2));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym2));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::T3(__nt) => {
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
        __sym1: &mut Option<(usize, Term, usize)>,
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T3((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym2));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym2));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T1((
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
        __sym1: &mut Option<(usize, Term, usize)>,
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T3((
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
pub use self::__parse__T2::parse_T2;

mod __parse__T3 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use expression::Predicate;
    use expression::Term;
    use expression::NotData;
    use expression::AndData;
    use expression::OrData;
    use expression::ImpliesData;
    use expression::IntegerComparisonData;
    use expression::UnsignedBitVectorData;
    use expression::VariableMappingData;
    use expression::UnaryExpressionData;
    use expression::BinaryExpressionData;
    use expression::IntegerComparisonOperator;
    use expression::IntegerUnaryOperator;
    use expression::IntegerBinaryOperator;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_T3<
        'input,
    >(
        input: &'input str,
    ) -> Result<Term, __ParseError<usize,(usize, &'input str),()>>
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
            (None, __Nonterminal::____T3((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        INT__BOP((usize, IntegerBinaryOperator, usize)),
        INT__COMP((usize, IntegerComparisonOperator, usize)),
        INT__UOP((usize, IntegerUnaryOperator, usize)),
        T1((usize, Term, usize)),
        T2((usize, Term, usize)),
        T3((usize, Term, usize)),
        ____INT__BOP((usize, IntegerBinaryOperator, usize)),
        ____INT__COMP((usize, IntegerComparisonOperator, usize)),
        ____INT__UOP((usize, IntegerUnaryOperator, usize)),
        ____T1((usize, Term, usize)),
        ____T2((usize, Term, usize)),
        ____T3((usize, Term, usize)),
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
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state2(input, __tokens, __sym0));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym0));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym0));
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
                __Nonterminal::T3(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __nt = __Nonterminal::____T3((
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

    pub fn __state2<
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
                __result = try!(__state9(input, __tokens, __sym1));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym1));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state3<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state4<
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym1));
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
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __tokens, __lookahead, __sym0, __sym1));
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
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __tokens, __lookahead, __sym1, __sym2));
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
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __nt = __Nonterminal::T1((
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

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __nt = __Nonterminal::T2((
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

    pub fn __state9<
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
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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

    pub fn __state10<
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
                __result = try!(__state9(input, __tokens, __sym1));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym1));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(input, __tokens, __sym1));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym1));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym1));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
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
            Some((_, (4, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::INT__UOP((
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

    pub fn __state12<
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __nt = __Nonterminal::T3((
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __nt = __Nonterminal::T3((
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

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, IntegerUnaryOperator, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1);
                let __nt = __Nonterminal::T2((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym2));
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(input, __tokens, __sym2));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym2));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym2));
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
                __Nonterminal::INT__UOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::T2(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::T3(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
        __sym1: &mut Option<(usize, Term, usize)>,
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T3((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (20, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __nt = __Nonterminal::INT__BOP((
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
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym2));
            }
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym2));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym2));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym2));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym2));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym2));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
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
                __Nonterminal::INT__BOP(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Term, usize)>,
        __sym1: &mut Option<(usize, IntegerBinaryOperator, usize)>,
        __sym2: &mut Option<(usize, Term, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T1((
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
        __sym1: &mut Option<(usize, Term, usize)>,
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
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T3((
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
pub use self::__parse__T3::parse_T3;
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
                        32 => /* ' ' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        33 => /* '!' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        37 => /* '%' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        38 => /* '&' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((19, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        60 => /* '<' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_state = 13;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 16;
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
                        32 => /* ' ' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 18;
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
                        61 => /* '=' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((19, __index + __ch.len_utf8()));
                            __current_state = 20;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        60 => /* '<' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
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
                            __current_match = Some((13, __index + 1));
                            __current_state = 23;
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
                            __current_match = Some((15, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 25;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((20, __index + __ch.len_utf8()));
                            __current_state = 18;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((19, __index + __ch.len_utf8()));
                            __current_state = 20;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
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
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, IntegerComparisonOperator, usize),
) -> IntegerComparisonOperator
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, IntegerUnaryOperator, usize),
) -> IntegerUnaryOperator
{
    (__0)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, IntegerBinaryOperator, usize),
) -> IntegerBinaryOperator
{
    (__0)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, t1, _): (usize, Term, usize),
    (_, op, _): (usize, IntegerBinaryOperator, usize),
    (_, t2, _): (usize, Term, usize),
) -> Term
{
    Term::BinaryExpression( BinaryExpressionData { op: op, t1: Box::new(t1), t2: Box::new(t2) } )
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, op, _): (usize, IntegerUnaryOperator, usize),
    (_, t, _): (usize, Term, usize),
) -> Term
{
    Term::UnaryExpression( UnaryExpressionData { op: op, t: Box::new(t) } )
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, &'input str, usize),
) -> Term
{
    Term::UnsignedBitVector( UnsignedBitVectorData { size: 64, value: u64::from_str(l).unwrap() } )
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> Term
{
    Term::VariableMapping( VariableMappingData { name: v.to_string(), var_type: "".to_string() } )
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Term
{
    t
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerComparisonOperator
{
    IntegerComparisonOperator::LessThan
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerComparisonOperator
{
    IntegerComparisonOperator::LessThanOrEqual
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerComparisonOperator
{
    IntegerComparisonOperator::GreaterThan
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerComparisonOperator
{
    IntegerComparisonOperator::GreaterThanOrEqual
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerComparisonOperator
{
    IntegerComparisonOperator::Equal
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerComparisonOperator
{
    IntegerComparisonOperator::NotEqual
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerUnaryOperator
{
    IntegerUnaryOperator::Negation
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerUnaryOperator
{
    IntegerUnaryOperator::BitwiseNot
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::Addition
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::Subtraction
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::Multiplication
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::Division
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::Modulo
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::BitwiseAnd
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::BitwiseOr
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::BitwiseXor
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::BitwiseLeftShift
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IntegerBinaryOperator
{
    IntegerBinaryOperator::BitwiseRightShift
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
