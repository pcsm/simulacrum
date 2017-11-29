use std::any::Any;
use std::fmt;

use super::{ExpectationId, MethodName};

pub type ExpectationResult = Result<(), ExpectationError>;

pub enum ExpectationError {
    CalledTooFewTimes(MethodName, i64),
    CalledTooManyTimes(MethodName, i64),
    CallNotExpected(MethodName),
    MismatchedParams(MethodName),
}

impl fmt::Display for ExpectationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ExpectationError::CalledTooFewTimes(name, times) => {
                write!(f, "{} was called {} times fewer than expected.", name, times)
            },
            &ExpectationError::CalledTooManyTimes(name, times) => {
                write!(f, "{} was called {} times more than expected.", name, times)
            },
            &ExpectationError::CallNotExpected(name) => {
                write!(f, "{} was called when not expected.", name)
            },
            &ExpectationError::MismatchedParams(name) => {
                write!(f, "{} was called with unexpected parameters.", name)
            }
        }
    }
}

/// An expectation that a method must be called. Also includes an optional
/// closure to produce return values, if necessary.
pub struct Expectation<I, O> {
    name: MethodName,
    constraints: Vec<Constraint>,
    return_fn: Option<Box<FnMut(I) -> O>>
}

impl<I, O> Expectation<I, O> {
    pub fn new(name: MethodName) -> Self {
        Expectation {
            name,
            constraints: Vec::new(),
            return_fn: None
        }
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.return_fn = Some(Box::new(return_behavior));
    }
}

pub trait ExpectationT {
    fn as_any(&mut self) -> &mut Any;

    fn constrain(&mut self, constraint: Constraint);

    fn verify(&mut self) -> ExpectationResult;
}

impl<I, O> ExpectationT for Expectation<I, O> where
    I: 'static,
    O: 'static
{
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn constrain(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    fn verify(&mut self) -> ExpectationResult {
        unimplemented!()
    }
}

pub enum Constraint {
    /// A method must be called with arguments that meet certain requirements.
    /// The `Any` in the `Box` is a closure that can be downcasted later and called.
    Params(Box<Any>),
    /// A method must be called a certain number of times
    Times(i64),
    /// For testing
    AlwaysPass,
    /// For testing
    AlwaysFail
}

/// A set of expectations that should be met at the same time.
///
/// Calling `Expectations.then()` creates a new era.
/// 
/// All expectations in an era must be met before the next era is evaluated.
pub struct ExpectationEra(Vec<ExpectationId>);

impl ExpectationEra {
    pub fn new() -> Self {
        ExpectationEra(Vec::new())
    }

    pub fn add(&mut self, id: ExpectationId) {
        self.0.push(id)
    }
}

/*
impl Expectation {
    pub fn validatemmm(&mut self) -> ExpectationResult {
        match self {
            &mut Expectation::CallArgs(key, boxed_t) => {
                boxed_t.validate()
            },
            &mut Expectation::CallTimes(key, times) => {
                match times {
                    x if x < 0 => Err(ExpectationError::CalledTooManyTimes(key, x.abs())),
                    x if x > 0 => Err(ExpectationError::CalledTooFewTimes(key, x)),
                    _ => Ok(())
                }
            },
        }
    }
}
*/


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