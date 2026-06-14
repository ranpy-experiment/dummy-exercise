use std::io::{self, Read};
use std::str::FromStr;

pub struct Scanner {
    tokens: Vec<String>,
    pos: usize,
}

#[allow(unused)]
impl Scanner {
    pub fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).expect("read stdin");
        let tokens = s.split_whitespace().map(String::from).collect();
        Scanner { tokens, pos: 0 }
    }

    // panicking version: returns T or panic on parse/error
    pub fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok.parse::<T>().expect("parse token")
    }

    // fallible version: returns Option<T>
    pub fn next_opt<T: FromStr>(&mut self) -> Option<T> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok.parse::<T>().ok()
    }

    // convenience for reading Vec<T> of known length
    pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}
