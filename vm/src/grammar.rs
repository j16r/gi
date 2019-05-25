use ast::{Node, Value};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::{Node, Value};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_28_29_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22false_22(&'input str),
        Term_22true_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        TermSWS(&'input str),
        TermtAtom(&'input str),
        TermtFn(&'input str),
        TermtInteger(&'input str),
        TermtLet(&'input str),
        TermtString(&'input str),
        Nt_28_3cExpression_3e_29(Box<Node>),
        Nt_28_3cExpression_3e_29_2a(::std::vec::Vec<Box<Node>>),
        Nt_28_3cExpression_3e_29_2b(::std::vec::Vec<Box<Node>>),
        NtAtom(Box<Node>),
        NtExpression(Box<Node>),
        NtFunctionApplication(Box<Node>),
        NtFunctionDefinition(Box<Node>),
        NtLet(Box<Node>),
        NtLiteralBool(Box<Node>),
        NtLiteralInteger(Box<Node>),
        NtLiteralString(Box<Node>),
        NtProgram(Box<Node>),
        Nt____Program(Box<Node>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 11, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 1
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 2
        0, 0, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 3
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 4
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 5
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 6
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 7
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 8
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 9
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 10
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 11
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 12
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 13
        20, 0, -6, 0, -6, -6, 0, -6, 0, -6, -6, -6, -6, -6,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 15
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 17
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 18
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 19
        0, 0, 26, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 20
        27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 23
        0, 0, 30, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 24
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 25
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 26
        0, 0, 32, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 27
        0, 0, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 28
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 29
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 30
        0, 0, 34, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 31
        0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 33
        0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 35
        0, 0, 0, 0, 12, 13, 0, 0, 0, 14, 15, 16, 17, 18,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0,
        // State 38
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 39
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -13,
        -24,
        -12,
        -11,
        -10,
        -7,
        -8,
        -9,
        -26,
        -23,
        -20,
        -19,
        -6,
        0,
        -21,
        0,
        -22,
        -25,
        0,
        0,
        0,
        -6,
        0,
        -4,
        -14,
        0,
        0,
        -5,
        -15,
        0,
        0,
        -18,
        0,
        0,
        0,
        0,
        0,
        -16,
        -17,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 2, 19, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 24, 2, 25, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 2, 29, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 31, 2, 25, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 27
        0, 0, 0, 2, 33, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 2, 29, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 2, 37, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 35
        0, 0, 0, 2, 38, 4, 5, 6, 7, 8, 9, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###""()""###,
            r###"")""###,
            r###""=""###,
            r###""false""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
            r###"SWS"###,
            r###"tAtom"###,
            r###"tFn"###,
            r###"tInteger"###,
            r###"tLet"###,
            r###"tString"###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Node>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (10, _) if true => 4,
                (13, _) if true => 5,
                (4, _) if true => 6,
                (5, _) if true => 7,
                (9, _) if true => 8,
                (6, _) if true => 9,
                (11, _) if true => 10,
                (8, _) if true => 11,
                (12, _) if true => 12,
                (7, _) if true => 13,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_7b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_7d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::TermSWS((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::TermtAtom((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::TermtFn((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::TermtInteger((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::TermtLet((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::TermtString((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Node>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expression>) = Expression => ActionFn(21);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpression_3e_29(__nt), __end));
                0
            }
            2 => {
                // (<Expression>)* =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpression_3e_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expression>)* = (<Expression>)+ => ActionFn(20);
                let __sym0 = __pop_Nt_28_3cExpression_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpression_3e_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expression>)+ = Expression => ActionFn(24);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpression_3e_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expression>)+ = (<Expression>)+, Expression => ActionFn(25);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cExpression_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpression_3e_29_2b(__nt), __end));
                2
            }
            6 => {
                // Atom = tAtom => ActionFn(14);
                let __sym0 = __pop_TermtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            7 => {
                // Expression = LiteralBool => ActionFn(4);
                let __sym0 = __pop_NtLiteralBool(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            8 => {
                // Expression = LiteralInteger => ActionFn(5);
                let __sym0 = __pop_NtLiteralInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            9 => {
                // Expression = LiteralString => ActionFn(6);
                let __sym0 = __pop_NtLiteralString(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            10 => {
                // Expression = Let => ActionFn(7);
                let __sym0 = __pop_NtLet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            11 => {
                // Expression = FunctionDefinition => ActionFn(8);
                let __sym0 = __pop_NtFunctionDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            12 => {
                // Expression = FunctionApplication => ActionFn(9);
                let __sym0 = __pop_NtFunctionApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            13 => {
                // Expression = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                4
            }
            14 => {
                // FunctionApplication = tAtom, "(", ")" => ActionFn(26);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFunctionApplication(__nt), __end));
                5
            }
            15 => {
                // FunctionApplication = tAtom, "(", (<Expression>)+, ")" => ActionFn(27);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Nt_28_3cExpression_3e_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtFunctionApplication(__nt), __end));
                5
            }
            16 => {
                // FunctionDefinition = tFn, tAtom, "(", ")", "{", Expression, "}" => ActionFn(28);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtExpression(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermtAtom(__symbols);
                let __sym0 = __pop_TermtFn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtFunctionDefinition(__nt), __end));
                6
            }
            17 => {
                // FunctionDefinition = tFn, tAtom, "(", (<Expression>)+, ")", "{", Expression, "}" => ActionFn(29);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_NtExpression(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_Nt_28_3cExpression_3e_29_2b(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermtAtom(__symbols);
                let __sym0 = __pop_TermtFn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtFunctionDefinition(__nt), __end));
                6
            }
            18 => {
                // Let = tLet, Atom, "=", Expression => ActionFn(11);
                let __sym3 = __pop_NtExpression(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_TermtLet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtLet(__nt), __end));
                7
            }
            19 => {
                // LiteralBool = "true" => ActionFn(17);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteralBool(__nt), __end));
                8
            }
            20 => {
                // LiteralBool = "false" => ActionFn(18);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteralBool(__nt), __end));
                8
            }
            21 => {
                // LiteralInteger = tInteger => ActionFn(16);
                let __sym0 = __pop_TermtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteralInteger(__nt), __end));
                9
            }
            22 => {
                // LiteralString = tString => ActionFn(15);
                let __sym0 = __pop_TermtString(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteralString(__nt), __end));
                10
            }
            23 => {
                // Program = "()" => ActionFn(1);
                let __sym0 = __pop_Term_22_28_29_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                11
            }
            24 => {
                // Program = Expression => ActionFn(2);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                11
            }
            25 => {
                // Program = Expression, Expression => ActionFn(3);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                11
            }
            26 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22false_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22false_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22true_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22true_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermSWS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermSWS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermtFn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermtFn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermtLet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermtLet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermtString<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermtString(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpression_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpression_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpression_3e_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Node>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpression_3e_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpression_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Node>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpression_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFunctionApplication<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFunctionApplication(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFunctionDefinition<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFunctionDefinition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteralBool<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteralBool(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteralInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteralInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteralString<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteralString(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;
mod __intern_token {
    #![allow(unused_imports)]
    use ast::{Node, Value};
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\\()",
                "^(?u:\\(\\))",
                "^(?u:\\))",
                "^(?u:=)",
                "^(?u:\\{)",
                "^(?u:\\})",
                "^(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ\u{300}-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙ-ՙա-և\u{591}-\u{5bd}\u{5bf}-\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}-\u{5c7}א-תװ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿ-ۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ-ߺࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࢽ\u{8d4}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}-\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ-ৼ\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}-\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}-\u{a51}ਖ਼-ੜਫ਼-ਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐ-ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b56}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ-ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ-ௐ\u{bd7}-\u{bd7}௦-௯\u{c00}-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞ-ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഃഅ-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ\u{dca}-\u{dca}\u{dcf}-\u{dd4}\u{dd6}-\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-\u{eb9}\u{ebb}-ຽເ-ໄໆ-ໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ-ༀ\u{f18}-\u{f19}༠-༩\u{f35}-\u{f35}\u{f37}-\u{f37}\u{f39}-\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}-\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗ-ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ-ᪧ\u{1ab0}-\u{1abe}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈ\u{1cd0}-\u{1cd2}\u{1cd4}-\u{1cf9}ᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ\u{20d0}-\u{20f0}ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ-ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻ-ꣻꣽ-ꣽ꤀-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}-\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}-\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨳\u{10a38}-\u{10a3a}\u{10a3f}-\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅐-\u{11173}𑅶-𑅶\u{11180}-𑇄\u{111ca}-\u{111cc}𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{1123e}𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133c}-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐\u{11357}-\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄-𑙄𑙐-𑙙𑚀-\u{116b7}𑛀-𑛉𑜀-𑜙\u{1171d}-\u{1172b}𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-\u{11a3e}\u{11a47}-\u{11a47}𑩐-𑪃𑪆-\u{11a99}𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}-\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾\u{16f8f}-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}-\u{1da75}\u{1da84}-\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-\u{1e94a}𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀\u{e0100}-\u{e01ef}])+",
                "^(?u:\")(?:(?u:[\u{0}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\")",
                "^((?u:0)|(?u:[1-9])(?u:[0-9])*)",
                "^(?u:[\t-\r - \u{85}-\u{85}\u{a0}-\u{a0}\u{1680}-\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}-\u{202f}\u{205f}-\u{205f}\u{3000}-\u{3000}])+",
                "^(?u:false)",
                "^(?u:fn)",
                "^(?u:let)",
                "^(?u:true)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\(\\))").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:\\{)").unwrap(),
                __regex::Regex::new("^(?u:\\})").unwrap(),
                __regex::Regex::new("^(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ\u{300}-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙ-ՙա-և\u{591}-\u{5bd}\u{5bf}-\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}-\u{5c7}א-תװ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿ-ۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ-ߺࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࢽ\u{8d4}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}-\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ-ৼ\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}-\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}-\u{a51}ਖ਼-ੜਫ਼-ਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐ-ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b56}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ-ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ-ௐ\u{bd7}-\u{bd7}௦-௯\u{c00}-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞ-ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഃഅ-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ\u{dca}-\u{dca}\u{dcf}-\u{dd4}\u{dd6}-\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-\u{eb9}\u{ebb}-ຽເ-ໄໆ-ໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ-ༀ\u{f18}-\u{f19}༠-༩\u{f35}-\u{f35}\u{f37}-\u{f37}\u{f39}-\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}-\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗ-ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ-ᪧ\u{1ab0}-\u{1abe}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈ\u{1cd0}-\u{1cd2}\u{1cd4}-\u{1cf9}ᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ\u{20d0}-\u{20f0}ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ-ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻ-ꣻꣽ-ꣽ꤀-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}-\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}-\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨳\u{10a38}-\u{10a3a}\u{10a3f}-\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅐-\u{11173}𑅶-𑅶\u{11180}-𑇄\u{111ca}-\u{111cc}𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{1123e}𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133c}-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐\u{11357}-\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄-𑙄𑙐-𑙙𑚀-\u{116b7}𑛀-𑛉𑜀-𑜙\u{1171d}-\u{1172b}𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-\u{11a3e}\u{11a47}-\u{11a47}𑩐-𑪃𑪆-\u{11a99}𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}-\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾\u{16f8f}-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}-\u{1da75}\u{1da84}-\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-\u{1e94a}𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀\u{e0100}-\u{e01ef}])+").unwrap(),
                __regex::Regex::new("^(?u:\")(?:(?u:[\u{0}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\")").unwrap(),
                __regex::Regex::new("^((?u:0)|(?u:[1-9])(?u:[0-9])*)").unwrap(),
                __regex::Regex::new("^(?u:[\t-\r - \u{85}-\u{85}\u{a0}-\u{a0}\u{1680}-\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}-\u{202f}\u{205f}-\u{205f}\u{3000}-\u{3000}])+").unwrap(),
                __regex::Regex::new("^(?u:false)").unwrap(),
                __regex::Regex::new("^(?u:fn)").unwrap(),
                __regex::Regex::new("^(?u:let)").unwrap(),
                __regex::Regex::new("^(?u:true)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 14 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Node>
{
    Box::new(Node::Nil)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, first, _): (usize, Box<Node>, usize),
    (_, rest, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    Box::new(Node::Cons(first, rest))
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, atom, _): (usize, Box<Node>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, expression, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    {
  Box::new(Node::Let(atom, expression))
}
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, arguments, _): (usize, ::std::vec::Vec<Box<Node>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Box<Node>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Node>
{
    {

  Box::new(
    Node::Function(
      name.into(),
      arguments.into_iter().rev().fold(Box::new(Node::Nil), |accumulator, argument| {
        Box::new(Node::Cons(Box::new(*argument), accumulator))
      }),
      body))
}
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, atom, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, expression, _): (usize, ::std::vec::Vec<Box<Node>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Node>
{
    {
  Box::new(
    Node::FunctionApplication(
      atom.into(),
      expression.into_iter().rev().fold(Box::new(Node::Nil), |accumulator, argument| {
        Box::new(Node::Cons(Box::new(*argument), accumulator))
      })))
}
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, value, _): (usize, &'input str, usize),
) -> Box<Node>
{
    Box::new(Node::Atom(value.into()))
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, value, _): (usize, &'input str, usize),
) -> Box<Node>
{
    {
  Box::new(Node::Value(Value::U8String(value[1..value.len() - 1].into())))
}
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, value, _): (usize, &'input str, usize),
) -> Box<Node>
{
    Box::new(Node::Value(Value::Integer32(value.parse().unwrap())))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Node>
{
    Box::new(Node::Value(Value::Bool(true)))
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Node>
{
    Box::new(Node::Value(Value::Bool(false)))
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Node>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Node>>, usize),
) -> ::std::vec::Vec<Box<Node>>
{
    v
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> Box<Node>
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Node>, usize),
) -> ::std::vec::Vec<Box<Node>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Node>>, usize),
    (_, e, _): (usize, Box<Node>, usize),
) -> ::std::vec::Vec<Box<Node>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Node>, usize),
) -> ::std::vec::Vec<Box<Node>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action21(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Node>>, usize),
    __1: (usize, Box<Node>, usize),
) -> ::std::vec::Vec<Box<Node>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> Box<Node>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<Box<Node>>, usize),
    __3: (usize, &'input str, usize),
) -> Box<Node>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action20(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, Box<Node>, usize),
    __6: (usize, &'input str, usize),
) -> Box<Node>
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Box<Node>>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Box<Node>, usize),
    __7: (usize, &'input str, usize),
) -> Box<Node>
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action20(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __4,
        __5,
        __6,
        __7,
    )
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
