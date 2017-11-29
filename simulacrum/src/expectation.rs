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
            },
            _ => write!(f, "Unknown error")
        }
    }
}

/// An expectation that a method must be called. Also includes an optional
/// closure to produce return values, if necessary.
pub struct Expectation {
    name: MethodName,
    call_exps: Vec<CallExpectation>,
    return_fn: Option<Box<Any>>
}

impl Expectation {
    pub fn new(name: MethodName) -> Self {
        Expectation {
            name,
            call_exps: Vec::new(),
            return_fn: None
        }
    }

    pub fn verify(&mut self) -> ExpectationResult {
        unimplemented!()
    }

    pub(crate) fn add(&mut self, c_exp: CallExpectation) {
        self.call_exps.push(c_exp);
    }

    pub(crate) fn set_return(&mut self, return_behavior: Box<Any>) {
        self.return_fn = Some(return_behavior);
    }
}

pub enum CallExpectation {
    /// A method must be called with arguments that meet certain requirements.
    /// The `Any` in the `Box` is a closure that can be downcasted later and called.
    Params(Box<Any>),
    /// A method must be called a certain number of times
    Times(i64),
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