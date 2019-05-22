use crate::fastate::*;
use unit::*;
use std::collections::HashSet;

pub struct Dfa {
    pub head: usize,
    pub tail: Vec<usize>,
    pub states: Vec<FaState>,
}

use std::collections::VecDeque;
use std::collections::HashMap;

fn get_epsilon_closure(t: Option<Vec<bool>>, states: &Vec<FaState>) -> Option<Vec<bool>> {
    match t {
        None => None,
        Some(t) => {
            let mut e_t = t.clone();
            let mut stack: Vec<usize> = Vec::new();
            for (i, b) in t.iter().enumerate() {
                match b {
                    true => stack.push(i),
                    false => {},
                }
            }
            loop {
                if let Some(t) = stack.pop() {
                    for tran in &states[t].trans {
                        match tran {
                            Tran::Char(_ch, _u) => {},
                            Tran::Epsilon(u)  => {
                                if e_t[*u] == false {
                                    e_t[*u] = true;
                                    stack.push(*u);
                                }
                            }
                        }
                    }
                } else {break;}
            }
            Some(e_t)
        }
    }
}
fn get_move(t: Vec<bool>, a: &Char, states: &Vec<FaState>) -> Option<Vec<bool>> {
    let mut m_t = None;

    for (i, &b) in t.iter().enumerate() {
        if b == true {
            for tran in &states[i].trans {
                if let Tran::Char(ch, to) = tran {
                    if *ch == *a {
                        if let None = m_t {
                            let mut tmp: Vec<bool> = Vec::new();
                            for _i in 0..states.len() {
                                tmp.push(false);
                            }
                            m_t = Some(tmp);
                        }

                        m_t = match m_t {
                            None => panic!(""),
                            Some(mut m_t) => {
                                m_t[*to] = true;
                                Some(m_t)
                            }
                        }
                    }
                }
            }
        }
    }

    m_t
}

impl Dfa {
    pub fn from_nfa(nfa_states: &Vec<FaState>, chars: HashSet<Char>) -> Dfa {
        let mut dfa = Dfa {
            head: 0,
            tail: vec![],
            states: [].iter().cloned().collect(),
        };
        dfa.states.push(FaState {
            index: dfa.states.len(),
            trans: [].iter().cloned().collect(),
            kind: FaStateType::Head,
        });

        let mut bool_states: Vec<bool> = Vec::new();
        for _i in 0..nfa_states.len() { bool_states.push(false); }
        bool_states[0] = true;
        bool_states = match get_epsilon_closure(Some(bool_states), &nfa_states) {
            None => panic!(""),
            Some(bool_states) => bool_states,
        };

        let mut raw_states: Vec<Vec<bool>> = Vec::new();
        raw_states.push(bool_states.clone());

        let mut map: HashMap<Vec<bool>, usize> = HashMap::new();
        map.insert(bool_states.clone(), 0);

        let mut done: HashSet<usize> = HashSet::new();


        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);

        loop {

            if let Some(t) = queue.pop_front() {
                done.insert(t);

                for ch in chars.iter() {
                    let bool_states = raw_states[t].clone();

                    if let Some(u) = get_epsilon_closure(get_move(bool_states, ch, &nfa_states), &nfa_states) {
                        let u_id = if let Some(u_id) = map.get(&u) {
                            *u_id
                        } else {
                            let u_id = dfa.states.len();

                            dfa.states.push(FaState {
                                index: u_id,
                                trans: [].iter().cloned().collect(),
                                kind: {
                                    if u[nfa_states.len() - 1] {
                                        dfa.tail.push(u_id);
                                        FaStateType::Tail
                                    } else {
                                        FaStateType::Normal
                                    }
                                },
                            });

                            raw_states.push(u.clone());

                            map.insert(u.clone(), u_id);

                            u_id
                        };

                        dfa.states[t].add_char_tran(ch.clone(), u_id);

                        if let None = done.get(&u_id) {
                            queue.push_back(u_id);
                        }
                    }
                }
            } else {break;}
        }

        dfa
    }

    pub fn to_string(&self) -> String {
        let mut ans = String::from("digraph dfa {\n");

        for state in self.states.iter() {
            ans += &state.to_string();
        }

        ans.push_str("}");

        ans
    }
}