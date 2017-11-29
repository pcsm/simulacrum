use std::any::Any;

use super::{ExpectationId, MethodName};

pub mod constraint;
pub mod result;

pub use self::constraint::Constraint;
pub use self::result::{ExpectationError, ExpectationResult};

/// An expectation that a method must be called. Also includes an optional
/// closure to produce return values, if necessary.
pub struct Expectation<I, O> where
    I: 'static
{
    name: MethodName,
    constraints: Vec<Constraint<I>>,
    return_fn: Option<Box<FnMut(I) -> O>>
}

impl<I, O> Expectation<I, O> where
    I: 'static
{
    pub fn new(name: MethodName) -> Self {
        Expectation {
            name,
            constraints: Vec::new(),
            return_fn: None
        }
    }

    pub(crate) fn constrain(&mut self, constraint: Constraint<I>) {
        self.constraints.push(constraint);
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.return_fn = Some(Box::new(return_behavior));
    }
}

pub trait ExpectationT {
    fn as_any(&mut self) -> &mut Any;

    fn verify(&mut self) -> ExpectationResult;
}

impl<I, O> ExpectationT for Expectation<I, O> where
    I: 'static,
    O: 'static
{
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn verify(&mut self) -> ExpectationResult {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let e: Expectation<(), ()> = Expectation::new("foo");

        assert_eq!(e.name, "foo", "Name of Constraint should be `foo`");
        assert_eq!(e.constraints.len(), 0, "Number of Constraints should be 0");
        assert!(e.return_fn.is_none(), "Return Closure Should Not Exist");
    }

    #[test]
    fn test_constrain() {
        let mut e: Expectation<(), ()> = Expectation::new("test");

        e.constrain(Constraint::AlwaysPass);

        assert_eq!(e.constraints.len(), 1, "Number of Constraints should be 1");
    }

    #[test]
    fn test_set_return() {
        let mut e: Expectation<(), i32> = Expectation::new("yaz");

        e.set_return(|_| 5);

        assert!(e.return_fn.is_some(), "Return Closure Should Exist");
        let mut f = e.return_fn.unwrap();
        assert_eq!(f(()), 5, "Return Closure Should return 5");
    }
}