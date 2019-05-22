use crate::fastate::*;
use unit::*;

#[derive(Clone)]
pub struct Nfa {
    pub states: Vec<FaState>,
    pub head: usize,
    pub tail: usize,
}

impl Nfa {
    pub fn add_offset(&mut self, offset: usize) {
        self.head += offset;
        self.tail += offset;
        for state in &mut self.states {
            state.add_offset(offset);
        }
    }

    fn from_concatenation(mut nfa_1: Nfa, mut nfa_2: Nfa) -> Nfa {
        nfa_2.add_offset(nfa_1.states.len());

        let old_tail = nfa_1.tail;
        let old_head = nfa_2.head;
        let tail = nfa_2.tail;

        nfa_1.states.append(&mut nfa_2.states);
        nfa_1.states[old_tail].add_epsilon_tran(old_head);

        nfa_1.tail = tail;

        nfa_1
    }

    fn from_alternation(mut nfa_1: Nfa, mut nfa_2: Nfa) -> Nfa {
        let len_1 = nfa_1.states.len();
        let len_2 = nfa_2.states.len();

        let head = FaState {
            index: 0,
            trans: [
                Tran::Epsilon(1),
                Tran::Epsilon(1 + len_1),
            ].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let tail = FaState {
            index: len_1 + len_2 + 1,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        nfa_1.add_offset(1);
        nfa_2.add_offset(len_1 + 1);

        nfa_1.states[nfa_1.tail - 1].add_epsilon_tran(tail.index);
        nfa_2.states[nfa_2.tail - 1 - len_1].add_epsilon_tran(tail.index);

        Nfa {
            head: 0,
            tail: tail.index,
            states: {
                let mut states: Vec<FaState> = Vec::new();

                states.push(head);
                states.append(&mut nfa_1.states);
                states.append(&mut nfa_2.states);
                states.push(tail);

                states
            }
        }
    }

    fn from_repeat_fromzero(mut nfa_1: Nfa) -> Nfa {
        let len_1 = nfa_1.states.len();

        let head = FaState {
            index: 0,
            trans: [
                Tran::Epsilon(1),
                Tran::Epsilon(1 + len_1),
            ].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let tail = FaState {
            index: len_1 + 1,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        nfa_1.add_offset(1);

        nfa_1.states[nfa_1.tail - 1].add_epsilon_tran(tail.index);
        nfa_1.states[nfa_1.tail - 1].add_epsilon_tran(1);

        Nfa {
            head: 0,
            tail: tail.index,
            states: {
                let mut states: Vec<FaState> = Vec::new();

                states.push(head);
                states.append(&mut nfa_1.states);
                states.push(tail);

                states
            }
        }
    }

    fn from_repeat_maybe(mut nfa_1: Nfa) -> Nfa {
        let len_1 = nfa_1.states.len();

        let head = FaState {
            index: 0,
            trans: [
                Tran::Epsilon(1),
                Tran::Epsilon(1 + len_1),
            ].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let tail = FaState {
            index: len_1 + 1,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        nfa_1.add_offset(1);

        nfa_1.states[nfa_1.tail - 1].add_epsilon_tran(tail.index);

        Nfa {
            head: 0,
            tail: tail.index,
            states: {
                let mut states: Vec<FaState> = Vec::new();

                states.push(head);
                states.append(&mut nfa_1.states);
                states.push(tail);

                states
            }
        }
    }

    fn from_repeat_exact(nfa_1: Nfa, times: usize) -> Nfa {
        let len_1 = nfa_1.states.len();

        let head = FaState {
            index: 0,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let tail = FaState {
            index: len_1 * times + 1,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let mut new_states: Vec<FaState> = Vec::new();

        new_states.push(head);

        for time in 0..times {
            let mut nfa_tmp = nfa_1.clone();
            nfa_tmp.add_offset(time * len_1 + 1);

            new_states.last_mut().unwrap().add_epsilon_tran(time * len_1 + 1);
            new_states.append(&mut nfa_tmp.states);
        }

        new_states.last_mut().unwrap().add_epsilon_tran(times * len_1 + 1);
        new_states.push(tail);

        Nfa {
            head: 0,
            tail: 1 + len_1 * times,
            states: new_states,
        }
    }

    fn from_repeat_from(nfa_1: Nfa, times: usize) -> Nfa {
        let len_1 = nfa_1.states.len();

        let head = FaState {
            index: 0,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let tail = FaState {
            index: len_1 * times + 1,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let mut new_states: Vec<FaState> = Vec::new();

        new_states.push(head);

        for time in 0..times {
            let mut nfa_tmp = nfa_1.clone();
            nfa_tmp.add_offset(time * len_1 + 1);

            new_states.last_mut().unwrap().add_epsilon_tran(time * len_1 + 1);
            new_states.append(&mut nfa_tmp.states);
        }

        new_states.last_mut().unwrap().add_epsilon_tran(times * len_1 + 1);
        new_states.last_mut().unwrap().add_epsilon_tran((times - 1) * len_1 + 1);
        new_states.push(tail);

        Nfa {
            head: 0,
            tail: 1 + len_1 * times,
            states: new_states,
        }
    }

    fn from_repeat_fromto(nfa_1: Nfa, m: usize, n: usize) -> Nfa {
        let len_1 = nfa_1.states.len();

        let head = FaState {
            index: 0,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let tail = FaState {
            index: len_1 * n + 1,
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Normal,
        };

        let mut new_states: Vec<FaState> = Vec::new();

        new_states.push(head);

        for time in 0..n {
            let mut nfa_tmp = nfa_1.clone();
            nfa_tmp.add_offset(time * len_1 + 1);

            new_states.last_mut().unwrap().add_epsilon_tran(time * len_1 + 1);
            new_states.append(&mut nfa_tmp.states);
        }

        new_states.last_mut().unwrap().add_epsilon_tran(n * len_1 + 1);
        new_states.push(tail);

        for i in m-1..n-1 {
            let index = (i + 1) * len_1;
            new_states[index].add_epsilon_tran(n * len_1 + 1);
        }

        Nfa {
            head: 0,
            tail: 1 + len_1 * m,
            states: new_states,
        }
    }

    pub fn from_expression(expression: &Vec<Unit>) -> Nfa {
        let units = expression;
        let mut nfas: Vec<Nfa> = Vec::new();
        for unit in units {
            match unit {
                Unit::Char(ch) => {
                    let head = FaState {
                        index: 0,
                        trans: [Tran::Char(ch.clone(), 1)].iter().cloned().collect(),
                        kind: FaStateType::Normal,
                    };
                    let tail = FaState {
                        index: 1,
                        trans: [].iter().cloned().collect(),
                        kind: FaStateType::Normal,
                    };

                    nfas.push(
                        Nfa {
                            states: vec![head, tail],
                            head: 0,
                            tail: 1,
                        }
                    )
                },
                Unit::Operator(Operator::Concatenation()) => {
                    let nfa_2 = nfas.pop().expect("Failed to get nfa");
                    let nfa_1 = nfas.pop().expect("Failed to get nfa");

                    nfas.push(Nfa::from_concatenation(nfa_1, nfa_2));
                },
                Unit::Operator(Operator::Alternation()) => {
                    let nfa_2 = nfas.pop().expect("Failed to get nfa");
                    let nfa_1 = nfas.pop().expect("Failed to get nfa");

                    nfas.push(Nfa::from_alternation(nfa_1, nfa_2));
                },
                Unit::Operator(Operator::Repeat(repeat)) => {
                    match repeat {
                        Repeat::Exact(times) => {
                            let nfa = nfas.pop().expect("Failed to get nfa");
                            nfas.push(Nfa::from_repeat_exact(nfa, *times));
                        },
                        Repeat::FromZero() => {
                            let nfa = nfas.pop().expect("Failed to get nfa");
                            nfas.push(Nfa::from_repeat_fromzero(nfa));
                        },
                        Repeat::From(from) => {
                            let nfa = nfas.pop().expect("Failed to get nfa");
                            nfas.push(Nfa::from_repeat_from(nfa, *from));
                        },
                        Repeat::FromTo(from, to) => {
                            let nfa = nfas.pop().expect("Failed to get nfa");
                            nfas.push(Nfa::from_repeat_fromto(nfa, *from, *to));
                        },
                        Repeat::Maybe() => {
                            let nfa = nfas.pop().expect("Failed to get nfa");
                            nfas.push(Nfa::from_repeat_maybe(nfa));
                        }
                    }
                },
                _ => {
                    panic!("unexpected char");
                },
            }
        }

        nfas.pop().expect("Generate Nfa Error.")
    }

    pub fn to_string(&self) -> String{
        let mut ans = String::from("digraph nfa {\n  node [shape=doublecircle]\n");

        for state in self.states.iter() {
            ans += &state.to_string();
        }

        ans.push_str("}");

        ans
    }


}