use std::{
    any::Any,
    io::{self, Write},
    print,
};

use crate::{solution::Solution, utils::Scanner};

pub struct ValidNumber {}

impl ValidNumber {
    fn is_valid_number(&self, num: String) -> bool {
        let mut p: char = '*';
        let mut e: bool = false;
        let mut d: bool = false;
        for (i, n) in num.chars().enumerate() {
            if n.is_ascii_alphabetic() {
                if i == 0 {
                    return false;
                }

                if !n.eq_ignore_ascii_case(&'e') {
                    return false;
                } else if e {
                    return false;
                } else {
                    e = true;
                }
            }

            if n.eq(&'.') {
                if !p.is_digit(10) || d || e {
                    return false;
                } else {
                    d = true;
                }
            }

            if n.eq(&'-') || n.eq(&'+') {
                if p.ne(&'*') && p.ne(&'e') && p.ne(&'E') {
                    return false;
                }
            }

            p = n.clone();
        }

        p.is_digit(10) || (p.eq(&'.') && num.len() != 1)
    }
}

impl Solution for ValidNumber {
    fn name(&self) -> &str {
        "valid_number"
    }

    fn solve(&self) -> Option<Box<dyn Any>> {
        let mut sc: Scanner = Scanner::new();

        print!("Enter the number: ");
        io::stdout().flush().unwrap();
        let num: String = sc.next();

        let res: bool = self.is_valid_number(num);
        Some(Box::new(res))
    }
}
