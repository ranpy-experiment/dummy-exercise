use std::{
    any::Any,
    io::{self, Write},
};

use crate::{solution::Solution, utils::Scanner};

pub struct RemoveDuplicateTwo {}

impl RemoveDuplicateTwo {
    fn remove_duplicates(&self, nums: &mut Vec<i32>) -> Vec<i32> {
        let total_size: usize = nums.len();
        let mut read_index: usize = 0;
        let mut write_index: usize = 0;
        while read_index < total_size {
            // todo@randeep: can instead start from 2
            if read_index < 2 {
                read_index += 1;
                write_index += 1;
                continue;
            }

            // todo@randeep: can just collapse into else block
            if nums[read_index] == nums[write_index - 2] {
                read_index += 1;
            } else {
                nums[write_index] = nums[read_index];
                write_index += 1;
                read_index += 1;
            }
            // read_index += 1;
        }

        nums.clone()
    }
}

impl Solution for RemoveDuplicateTwo {
    fn name(&self) -> &str {
        "remove_duplicate_from_sorted_array_ii"
    }

    fn solve(&self) -> Option<Box<dyn Any>> {
        let mut sc: Scanner = Scanner::new();

        print!("Enter the size of the array: ");
        io::stdout().flush().unwrap();
        let n: i32 = sc.next();

        print!("Enter the elements of the list in order: ");
        io::stdout().flush().unwrap();
        let mut nums: Vec<i32> = sc.read_vec(n as usize);

        let res: Vec<i32> = self.remove_duplicates(&mut nums);

        Some(Box::new(res))
    }
}
