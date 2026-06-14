use std::io;
use std::str::FromStr;

pub struct Scanner {
    tokens: Vec<String>,
    pos: usize,
}

#[allow(unused)]
impl Scanner {
    pub fn from_string(input: &str) -> Self {
        let tokens = input.split_whitespace().map(String::from).collect();
        Scanner { tokens, pos: 0 }
    }

    pub fn new() -> Self {
        Scanner {
            tokens: Vec::new(),
            pos: 0,
        }
    }

    fn fill(&mut self) -> bool {
        loop {
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => return false,
                Ok(_) => {
                    self.tokens = line.split_whitespace().map(String::from).collect();
                    self.pos = 0;
                    if !self.tokens.is_empty() {
                        return true;
                    }
                }
                Err(e) => panic!("read error: {}", e),
            }
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        if self.pos >= self.tokens.len() {
            if !self.fill() {
                panic!("unexpected EOF");
            }
        }
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok.parse::<T>().expect("parse token")
    }

    pub fn next_opt<T: FromStr>(&mut self) -> Option<T> {
        if self.pos >= self.tokens.len() {
            if !self.fill() {
                return None;
            }
        }
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok.parse::<T>().ok()
    }

    pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}
