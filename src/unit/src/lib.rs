#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Char {
    Single(char),
    Set(Vec<char>),
    Range(char, char),
    Ranges(Vec<(char, char)>),
    Not(char, char),
    But(char, char, char, char),
}

impl Char {
    pub fn is_match(&self, ch: char) -> bool {
        match self {
            Char::Single(a) => {
                ch == *a
            },
            Char::Set(char_vec) => {
                for ch_ in char_vec.iter() {
                    if *ch_ == ch {
                        return true;
                    }
                }
                return false;
            },
            Char::Range(a, b) => {
                (*a <= ch) && (ch <= *b)
            },
            Char::Not(a, b) => {
                !((*a <= ch) && (ch <= *b))
            },
            Char::But(a, b, c, d) => {
                (*a <= ch) && (ch <= *b) && (!((*c <= ch) && (ch <= *d)))
            },
            Char::Ranges(char_ranges) => {
                for (a, b) in char_ranges.iter() {
                    if *a <= ch && ch <= *b {
                        return true
                    }
                }
                return false
            }
        }
    }

    pub fn from_str(raw_str: &str) -> Option<Char> {
        if let Ok(char_class) = scan_fmt!(raw_str ,"[:{}:]" , String) {
            match &char_class[..] {
                "alnum" => Some(Char::Ranges(vec![('a','z'),('A','Z'),('0','9')])),
                "alpha" => Some(Char::Ranges(vec![('a','z'),('A','Z')])),
                "lower" => Some(Char::Range('a', 'z')),
                "upper" => Some(Char::Range('A', 'Z')),
                "blank" => Some(Char::Set(vec![' ', '\t'])),
                "space" => Some(Char::Single('a')),
                "cntrl" => Some(Char::Single('a')),
                "digit" => Some(Char::Range('0', '9')),
                //"xdigit" => Some(Char::Single('a')),
                "graph" => Some(Char::Single('a')),
                "print" => Some(Char::Range(' ', '~')),
                "punct" => Some(Char::Single('a')),
                _ => None,
            }
        } else if let Ok((a,b)) = scan_fmt!(raw_str ,"[^{}-{}]", char, char) {
            Some(Char::Not(a,b))
        } else if let Ok((a,b,c,d)) = scan_fmt!(raw_str ,"[{}-{}-[{}-{}]]", char, char, char, char) {
            Some(Char::But(a,b,c,d))
        } else if let Ok((a,b)) = scan_fmt!(raw_str ,"[{}-{}]", char, char) {
            Some(Char::Range(a,b))
        } else if let Ok(chars) = scan_fmt!(raw_str ,"[{}]" , String) {
            let mut char_set = Vec::<char>::new();
            for ch in chars.chars() {
                char_set.push(ch);
            }
            Some(Char::Set(char_set))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Char::Single(a) => format!("[{}]", a),
            Char::Set(set) => {
                let mut chars = String::new();
                for ch in set.iter() {
                    chars.push(*ch);
                }
                format!("[{}]", chars)
            },
            Char::Range(a,b) => format!("[{}-{}]", a, b),
            Char::Not(a,b) => format!("[^{}-{}]", a, b),
            Char::But(a,b,c,d) => format!("[{}-{}-[{}-{}]]", a, b, c, d),
            Char::Ranges(ranges) => {
                let mut res = String::from("[");
                for (a, b) in ranges {
                    res += &format!("{}-{},", *a, *b);
                }
                res += "}";

                res
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Repeat {
    Exact(usize),
    FromZero(),
    From(usize),
    FromTo(usize, usize),
    Maybe(),
}

impl Repeat {
    pub fn from_str(raw_str: &str) -> Option<Repeat> {
        if let Ok((a, b)) = scan_fmt!(raw_str ,"{{{},{}}}" , usize, usize) {
            Some(Repeat::FromTo(a, b))
        } else if let Ok(a) = scan_fmt!(raw_str ,"{{{},}}" , usize) {
            Some(Repeat::From(a))
        } else if let Ok(a) = scan_fmt!(raw_str ,"{{{}}}" , usize) {
            Some(Repeat::Exact(a))
        } else if raw_str == "*" {
            Some(Repeat::FromZero())
        } else if raw_str == "+" {
            Some(Repeat::From(1))
        } else if raw_str == "?" {
            Some(Repeat::Maybe())
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Repeat::Exact(times) => format!("{{{}}}", times),
            Repeat::FromZero() => format!("*"),
            Repeat::From(a) => format!("{{{},}}", a),
            Repeat::FromTo(a,b) => format!("{{{},{}}}", a, b),
            Repeat::Maybe() => format!("?"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Operator {
    LeftParenthese(),
    RightParenthese(),
    Alternation(),
    Concatenation(),
    Repeat(Repeat),
}

#[derive(PartialEq, Clone)]
pub enum Unit {
    Char(Char),
    Operator(Operator),
}

impl Unit {
    pub fn get_pivot(&self) -> u32 {
        match self {
            Unit::Operator(op) => match op {
                Operator::LeftParenthese()       => 1,
                Operator::RightParenthese()      => 2,
                Operator::Alternation()          => 3,
                Operator::Concatenation()        => 4,
                Operator::Repeat(_rp)            => 5,
            },
            Unit::Char(_ch) => 0,
        }
    }
}