use crate::Mix;
use std::fmt::Debug;

pub trait TransitionItem: Mix + Clone + Debug + Sized + PartialEq {}

impl<T: Mix + Clone + Debug + Sized + PartialEq> TransitionItem for T {}
