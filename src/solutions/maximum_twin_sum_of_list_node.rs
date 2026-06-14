use crate::solution::Solution;
use crate::types::ListNode;
use crate::utils::Scanner;
use std::any::Any;
use std::io;
use std::io::Write;

/*
 * In a linked list of size n, where n is even,
 * the ith node (0-indexed) of the linked list
 * is known as the twin of the (n-1-i)th node,
 *      if 0 <= i <= (n / 2) - 1.
 *
 * For example,
 *      if n = 4, then node 0 is the twin of node 3,
 *      and node 1 is the twin of node 2.
 *      These are the only nodes with twins for n = 4.
 *
 * The twin sum is defined as the sum of a node and its twin.
 * Given the head of a linked list with even length,
 * return the maximum twin sum of the linked list.
 */

pub struct MaximumTwinSum {}

impl MaximumTwinSum {
    fn pair_sum(&self, head: Option<Box<ListNode>>) -> i32 {
        return head.unwrap().val;
    }
}

impl Solution for MaximumTwinSum {
    fn name(&self) -> &str {
        "maximum_twin_sum_of_list_node"
    }

    fn solve(&self) -> Option<Box<dyn Any>> {
        let mut sc: Scanner = Scanner::new();

        print!("Enter the size of the linked list: ");
        io::stdout().flush().unwrap(); // show message before taking input
        let n: i32 = sc.next();

        print!("Enter the elements of the list in order: ");
        io::stdout().flush().unwrap(); // show message before taking input
        let elems: Vec<i32> = sc.read_vec(n as usize);

        let mut head_node: Option<Box<ListNode>> = None;
        for &el in elems.iter().rev() {
            let mut node: Box<ListNode> = Box::new(ListNode::new(el));
            node.next = head_node;
            head_node = Some(node);
        }

        let res: i32 = self.pair_sum(head_node);

        Some(Box::new(res))
    }
}
