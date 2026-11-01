use std::io::{self, Write};

use crate::{solution::Solution, utils::Scanner};

pub struct MinMovesEqualArray {}

impl MinMovesEqualArray {
    fn min_moves_to_equal_array(&self, nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let min_val: i32 = nums.iter().min().copied().unwrap_or(0);
        let total_val: i32 = nums.iter().sum();

        /*
         * because all that you can do is increment
         * therefore, the min value needs to catch up
         * to the max, and then adjust for the collateral
         */
        total_val - (n as i32 * min_val)
    }
}

impl Solution for MinMovesEqualArray {
    fn name(&self) -> &str {
        "minimum_moves_to_equal_array"
    }

    fn solve(&self) -> Option<Box<dyn std::any::Any>> {
        let mut sc: Scanner = Scanner::new();

        print!("Enter size of the array: ");
        io::stdout().flush().unwrap();
        let n: i32 = sc.next();

        print!("Enter elements of the array in order: ");
        io::stdout().flush().unwrap();
        let nums: Vec<i32> = sc.read_vec(n as usize);

        let res: i32 = self.min_moves_to_equal_array(nums);

        Some(Box::new(res))
    }
}
