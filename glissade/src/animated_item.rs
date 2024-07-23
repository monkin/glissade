use crate::Mix;

pub trait AnimatedItem: Mix + Clone + PartialEq {}

impl<T: Mix + Clone + PartialEq> AnimatedItem for T {}
