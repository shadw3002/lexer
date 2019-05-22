use unit::*;
use std::collections::HashSet;

pub struct Expression {
    pub units: Vec<Unit>,
}

impl Expression {
    pub fn get_chars(&self) -> HashSet<Char> {
        let mut ans: HashSet<Char> = HashSet::new();
        for u in &self.units {
            if let Unit::Char(ch) = u {
                ans.insert(ch.clone());
            }
        }

        ans
    }

    pub fn str_to_inffix_exp(regex: &String) -> Vec<Unit> {
        let mut units: Vec<Unit> = Vec::new();
        let mut stack = String::new();
        let mut count: u32 = 0;

        let mut escape_mode: bool = false;
        let mut escape_str = String::new();

        for ch in regex.chars() {
            if escape_mode {
                escape_str.push(ch);

            } else {
                match ch {
                    '[' |
                    '{' => {
                        count += 1;
                        stack.push(ch);
                    },
                    ']' => {
                        count -= 1;
                        stack.push(ch);

                        if count == 0 {
                            units.push(match Char::from_str(&stack) {
                                Some(ch) => Unit::Char(ch),
                                None => panic!("error"),
                            });
                            stack.clear();
                        }
                    },
                    '}' => {
                        count -= 1;
                        stack.push(ch);

                        if count == 0 {
                            units.push(match Repeat::from_str(&stack) {
                                Some(rp) => Unit::Operator(Operator::Repeat(rp)),
                                None => panic!("error"),
                            });
                            stack.clear();
                        }
                    },
                    '\\' => {
                        escape_mode = true;
                    },
                    _   => {
                        if count == 0 {
                            units.push(
                                match ch {
                                    '(' => Unit::Operator(Operator::LeftParenthese()),
                                    ')' => Unit::Operator(Operator::RightParenthese()),
                                    '|' => Unit::Operator(Operator::Alternation()),
                                    '*' => Unit::Operator(Operator::Repeat(Repeat::FromZero())),
                                    '+' => Unit::Operator(Operator::Repeat(Repeat::From(1))),
                                    '?' => Unit::Operator(Operator::Repeat(Repeat::Maybe())),
                                    _   => Unit::Char(Char::Single(ch)),
                                }
                            );
                        } else {
                            stack.push(ch);
                        }
                    },
                }
            }

        }

        let tmp =  units;
        let mut units: Vec<Unit> = Vec::new();
        for u in tmp.iter() {
            if let Some(top) = units.last() {
                match top {
                    Unit::Operator(t_op) => {
                        match t_op {
                            Operator::Repeat(_rp) => {
                                match u {
                                    Unit::Operator(u_op) => {
                                        match u_op {
                                            Operator::LeftParenthese() => {
                                                units.push(Unit::Operator(Operator::Concatenation()));
                                            },
                                            _ => {},
                                        }
                                    },
                                    Unit::Char(_u_ch) => {
                                        units.push(Unit::Operator(Operator::Concatenation()));
                                    }
                                }
                            },
                            Operator::RightParenthese() => {
                                match u {
                                    Unit::Operator(u_op) => {
                                        match u_op {
                                            Operator::LeftParenthese() => {
                                                units.push(Unit::Operator(Operator::Concatenation()));
                                            },
                                            _ => {},
                                        }
                                    },
                                    Unit::Char(_u_ch) => {
                                        units.push(Unit::Operator(Operator::Concatenation()));
                                    }
                                }
                            }
                            _=> {},
                        }
                    },
                    Unit::Char(_t_ch) => {
                        match u {
                            Unit::Operator(u_op) => {
                                match u_op {
                                    Operator::LeftParenthese() => {
                                        units.push(Unit::Operator(Operator::Concatenation()));
                                    },
                                    _ => {},
                                }
                            },
                            Unit::Char(_u_ch) => {
                                units.push(Unit::Operator(Operator::Concatenation()));
                            },
                        }
                    }
                }
            }

            units.push(u.clone());
        }

        units
    }

    pub fn inffix_to_suffix(regex: Vec<Unit>) -> Vec<Unit> {
        let mut ans: Vec<Unit> = Vec::new();
        let mut stack: Vec<Unit> = Vec::new();

        let mut back_reference_stack: Vec<Vec<Vec<Unit>>> = Vec::new();
        // let mut back_reference_point: usize = 0;

        for u in regex.iter() {
            let pivot: u32 = u.get_pivot();
            match pivot {
                0 => ans.push(u.clone()),
                _ => {
                    if stack.is_empty() {
                        stack.push(u.clone());
                    } else if *u == Unit::Operator(Operator::LeftParenthese()) {
                        stack.push(u.clone());
                    } else if *u == Unit::Operator(Operator::RightParenthese()) {
                        loop {
                            ans.push(if let Some(unit) = stack.pop() {
                                match unit {
                                    Unit::Operator(Operator::LeftParenthese()) => break,
                                    _ => unit,
                                }
                            } else { break; });
                        }

                        back_reference_stack.pop();
                    } else {
                        loop {
                            let stk_pivot: u32 = {
                                if let Some(unit) = stack.last() {
                                    unit
                                }
                                else {break;}
                            }.get_pivot();

                            if stk_pivot <= pivot {break;}
                            ans.push(stack.pop().unwrap());
                        }
                        stack.push(u.clone());
                    }
                },
            }
        }

        loop {
            if let Some(unit) = stack.pop() {
                ans.push(unit);
            } else { break; }
        }

        ans
    }

    pub fn to_string(&self) -> String {
        let mut res: String = String::new();

        for u in self.units.iter() {
            match u {
                Unit::Operator(op) => {
                    match op {
                        Operator::LeftParenthese() => res.push('('),
                        Operator::RightParenthese() => res.push(')'),
                        Operator::Alternation() => res.push('|'),
                        Operator::Concatenation() => res.push('.'),
                        Operator::Repeat(rp) => res += &rp.to_string(),
                    }
                },
                Unit::Char(ch) => {
                    res += &ch.to_string();
                }
            }
        }

        res
    }

    pub fn from_str(raw_str: &String) -> Expression {
        let inffix_exp = Expression::str_to_inffix_exp(raw_str);

        let suffix_exp = Expression::inffix_to_suffix(inffix_exp);

        Expression {
            units: suffix_exp,
        }
    }
}