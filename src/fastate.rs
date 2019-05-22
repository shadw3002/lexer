use std::collections::HashSet;
use unit::*;

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum Tran {
    Char(Char,usize),
    Epsilon(usize),
}

#[derive(Clone)]
pub enum FaStateType {
    Head,
    Tail,
    Normal,
}

#[derive(Clone)]
pub struct FaState {
    pub index: usize,
    pub trans: HashSet<Tran>,
    pub kind: FaStateType,
}

impl FaState {
    pub fn add_char_tran(&mut self, ch: Char, to: usize) {
        self.trans.insert(Tran::Char(ch, to));
    }

    pub fn add_epsilon_tran(&mut self, to: usize) {
        self.trans.insert(Tran::Epsilon(to));
    }

    pub fn to_string(&self) -> String {
        let mut ans = String::new();

        match self.kind {
            FaStateType::Head => {
                ans += &format!(
                    "  {} [shape=Msquare]\n",
                    self.index,
                );
            },
            FaStateType::Tail => {
                ans += &format!(
                    "  {} [shape=doublecircle]\n",
                    self.index,
                );
            },
            FaStateType::Normal => {
                ans += &format!(
                    "  {} [shape=circle]\n",
                    self.index,
                );
            },
        };

        for tran in self.trans.iter() {
            match tran {
                Tran::Char(ch, to) => {
                    ans += &format!(
                        "  {} -> {} [label=\"{}\"]\n",
                        self.index,
                        to,
                        ch.to_string(),
                    );
                },
                Tran::Epsilon(to) => {
                    ans += &format!(
                        "  {} -> {} [label=\"ε\"]\n",
                        self.index,
                        to,
                    );
                },
            }
        }

        ans
    }

    pub fn add_offset(&mut self, offset: usize) {
        let mut new_trans: HashSet<Tran> = HashSet::new();

        self.index += offset;
        for tran in &self.trans {
            match tran {
                Tran::Char(ch, to) => new_trans.insert(Tran::Char(ch.clone(), *to + offset)),
                Tran::Epsilon(to)  => new_trans.insert(Tran::Epsilon(*to + offset)),
            };
        }

        self.trans = new_trans;
    }
}