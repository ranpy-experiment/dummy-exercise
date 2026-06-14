use std::any::Any;

pub trait Solution {
    fn name(&self) -> &str;
    fn solve(&self) -> Option<Box<dyn Any>>;
}
