use core::fmt::{Debug, Display, Formatter};
use std::ops::{ControlFlow, FromResidual, Try};
use crate::leblanc::rustblanc::outcome::Outcome::Failure;

#[derive(Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Outcome<T> {
    Success(T),
    #[default]
    Failure
}

impl<T> Outcome<T> {
    pub fn determine(item: T, predicate: fn(&T) -> bool) -> Outcome<T> {
        match predicate(&item) {
            true => Outcome::Success(item),
            false => Failure
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Outcome::Success(t) => t,
            Failure => panic!("Called Outcome::unwrap() on an 'Failure' value")
        }
    }

    pub fn map<R>(self, map: fn(T) -> R) -> Outcome<R> {
        if let Outcome::Success(success) = self {
            Outcome::Success(map(success))
        } else {
            Failure
        }
    }

    pub fn unwrap_or(self, other: T) -> T {
        match self {
            Outcome::Success(success) => success,
            Failure => other
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            Outcome::Success(_) => true,
            Failure => false
        }
    }

    pub fn is_fail(&self) -> bool {
        match self {
            Outcome::Success(_) => false,
            Failure => true
        }
    }
}

impl<T: Display> Display for Outcome<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s: String = match self {
            Outcome::Success(t) => format!("Success({})", t),
            Outcome::Failure => String::from("Failure")
        };
        write!(f, "{}", s)
    }
}

impl <T: Debug> Debug for Outcome<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Outcome::Success(t) => {
                f.debug_struct("Outcome")
                    .field("Success", t)
                    .finish()
            }
            Outcome::Failure => {
                f.debug_tuple("Outcome")
                    .field(&String::from("Failure"))
                    .finish()
            }
        }
    }
}

impl<T> Try for Outcome<T> {
    type Output = T;
    type Residual = ();

    fn from_output(output: Self::Output) -> Self {
        Outcome::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Outcome::Success(t) => ControlFlow::Continue(t),
            Failure => ControlFlow::Break(())
        }
    }
}

impl<T> FromResidual for Outcome<T> {
    fn from_residual(_residual: <Self as Try>::Residual) -> Self {
        Failure
    }
}
