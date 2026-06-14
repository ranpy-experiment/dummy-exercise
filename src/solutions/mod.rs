pub mod collatz_sequence;
pub mod maximum_twin_sum_of_list_node;

pub use collatz_sequence::CollatzSequence;
pub use maximum_twin_sum_of_list_node::MaximumTwinSum;

pub fn all_solutions() -> Vec<Box<dyn crate::solution::Solution>> {
    vec![Box::new(MaximumTwinSum {}), Box::new(CollatzSequence {})]
}
