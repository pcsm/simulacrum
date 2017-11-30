use std::any::Any;

use super::{ExpectationId, MethodName};

pub use super::constraint::Constraint;
pub use self::result::{ExpectationError, ExpectationResult};

pub mod result;

/// An expectation that a method must be called. Also includes an optional
/// closure to produce return values, if necessary.
pub struct Expectation<I, O> where
    I: 'static
{
    name: MethodName,
    constraints: Vec<Box<Constraint<I>>>,
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

    pub(crate) fn constrain<C>(&mut self, constraint: C) where
        C: Constraint<I> + 'static
    {
        self.constraints.push(Box::new(constraint));
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.return_fn = Some(Box::new(return_behavior));
    }
}

pub trait ExpectationT {
    fn as_any(&mut self) -> &mut Any;

    fn verify(&self) -> ExpectationResult;
}

impl<I, O> ExpectationT for Expectation<I, O> where
    I: 'static,
    O: 'static
{
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn verify(&self) -> ExpectationResult {

        for constraint in self.constraints.iter() {
            let r = constraint.verify();
            if r.is_err() {
                let e = ExpectationError {
                    constraint_err: r.unwrap_err(),
                    method_name: self.name
                };
                return Err(e);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constraint::ConstraintError;
    use constraint::stock::always::{AlwaysFail, AlwaysPass};

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

        e.constrain(AlwaysPass);

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

    #[test]
    fn test_verify_pass() {
        let mut e: Expectation<(), ()> = Expectation::new("zonk");

        e.constrain(AlwaysPass);
        e.constrain(AlwaysPass);
        let r = e.verify();

        assert!(r.is_ok(), "Expectation should pass");
    }

    #[test]
    fn test_verify_fail() {
        let mut e: Expectation<(), ()> = Expectation::new("boop");

        e.constrain(AlwaysPass);
        e.constrain(AlwaysFail); // Will cause Expectation to fail
        let r = e.verify();

        assert!(r.is_err(), "Expectation should fail");
        let r = r.unwrap_err();
        assert_eq!(r.method_name, "boop", "Expectation error should have the correct method name");
        assert_eq!(r.constraint_err, ConstraintError::AlwaysFail, "Expectation error should contain the correct Constraint error");
    }
}