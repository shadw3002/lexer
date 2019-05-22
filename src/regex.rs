use crate::expression::Expression;
use crate::fastate::*;
use crate::nfa::*;
use crate::dfa::*;



pub struct Regex {
    expression: Expression,
    nfa: Nfa,
    dfa: Dfa,
}

impl Regex {
    pub fn to_strings(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        res.push(self.expression.to_string());
        res.push(self.nfa.to_string());
        res.push(self.dfa.to_string());

        res
    }

    pub fn from(raw_str: &String) -> Regex {
        let expression = Expression::from_str(raw_str);
        let chars = expression.get_chars();
        let nfa = Nfa::from_expression(&expression.units);
        let dfa = Dfa::from_nfa(&nfa.states, chars);


        Regex {
            expression: expression,
            nfa: nfa,
            dfa: dfa,
        }
    }
    pub fn match_next_state(&self, cur_state: usize, ch: char) -> Option<usize> {
        for tran in self.dfa.states[cur_state].trans.iter() {
            match tran {
                Tran::Char(ch_, to) => {
                    if ch_.is_match(ch) {
                        return Some(*to)
                    }
                },
                Tran::Epsilon(_to) => panic!("epsilon tran should not be found in dfa"),
            }
        }


        None
    }

    pub fn matcher(&self, content: &str, is_greed : bool) -> Option<usize> {
        let mut cur_state: usize = self.dfa.head;
        let mut last_accepted_state: Option<usize> = None;

        for (i, ch) in content.chars().enumerate() {
            if let Some(new_state) = self.match_next_state(cur_state, ch) {
                cur_state = new_state;
            } else {
                break;
            }

            if let FaStateType::Tail = self.dfa.states[cur_state].kind {
                if is_greed == false {
                    return Some(i)
                }
                last_accepted_state = Some(i)
            }
        }

        last_accepted_state
    }

    pub fn grep(&self, content: &str, is_greed : bool) -> Option<(usize, usize)> {
        if content.len() == 0 {
            return None
        }

        for i in 0..content.len() {
            if let Some(match_index) = self.matcher(&content[i..], is_greed) {
                return Some((i, i + match_index + 1))
            }
        }

        return None
    }

    pub fn grep_all(&self, content: &str, is_greed : bool) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();

        if content.len() == 0 {
            return res
        }

        for i in 0..content.len() {
            if let Some(match_index) = self.matcher(&content[i..], is_greed) {
                res.push((i, i + match_index + 1));
            }
        }

        return res
    }

    pub fn grep_not_overlapped(&self, content: &str, is_greed : bool) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();

        if content.len() == 0 {
            return res
        }

        for i in 0..content.len() {
            if let Some((_a, b)) = res.last() {
                if i < *b {
                    continue;
                }
            }

            if let Some(match_index) = self.matcher(&content[i..], is_greed) {
                res.push((i, i + match_index + 1));
            }
        }

        return res
    }


}