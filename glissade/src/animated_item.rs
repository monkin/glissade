use crate::Mix;
use std::fmt::Debug;

pub trait AnimatedItem: Mix + Clone + Sized + PartialEq {}

impl<T: Mix + Clone + Debug + Sized + PartialEq> AnimatedItem for T {}
