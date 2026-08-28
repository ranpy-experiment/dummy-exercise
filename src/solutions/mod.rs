pub mod collatz_sequence;
pub mod maximum_twin_sum_of_list_node;
pub mod remove_duplicate_from_sroted_array;

pub use collatz_sequence::CollatzSequence;
pub use maximum_twin_sum_of_list_node::MaximumTwinSum;
pub use remove_duplicate_from_sroted_array::RemoveDuplicateTwo;

pub fn all_solutions() -> Vec<Box<dyn crate::solution::Solution>> {
    return vec![
        Box::new(MaximumTwinSum {}),
        Box::new(CollatzSequence {}),
        Box::new(RemoveDuplicateTwo {}),
    ];
}
