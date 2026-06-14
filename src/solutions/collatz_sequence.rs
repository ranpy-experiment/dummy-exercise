use std::io::Write;

use crate::solution::Solution;
use crate::utils::Scanner;

pub struct CollatzSequence {}

impl CollatzSequence {
    fn collatz_sequence(&self, num: i32) -> i32 {
        if num <= 0 {
            panic!("Initial number should be greater than 0!!")
        }

        let mut n: i32 = num;
        let mut counter: i32 = 0;

        print!("{} -> ", n);
        counter += 1;

        while n > 1 {
            if n % 2 == 0 {
                // even
                n = n / 2;
            } else {
                // odd
                n = 3 * n + 1;
            }

            counter += 1;
            print!("{} -> ", n);
        }

        print!("\n");
        return counter;
    }
}

impl Solution for CollatzSequence {
    fn name(&self) -> &str {
        "length_of_collatz_sequence"
    }

    fn solve(&self) -> Option<Box<dyn std::any::Any>> {
        let input = crate::STDIN_INPUT.get().expect("stdin not initialized");
        let mut sc = Scanner::from_string(input);

        print!("Enter the number: ");
        std::io::stdout().flush().unwrap(); // show message before taking input
        let num: i32 = sc.next();

        let res: i32 = self.collatz_sequence(num);

        Some(Box::new(res))
    }
}
