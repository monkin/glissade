use crate::Mix;

pub trait AnimatedItem: Mix + Clone + Sized + PartialEq {}

impl<T: Mix + Clone + Sized + PartialEq> AnimatedItem for T {}
