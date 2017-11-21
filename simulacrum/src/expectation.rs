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

pub enum Expectation {
    /// Expectations that must all be met
    Group(Vec<ExpectationId>),
    /// A method must be called. The `Option<Box<Any>>` is a closure to produce return
    /// values, if necessary.
    Call(MethodName, Vec<CallExpectation>, Option<Box<Any>>),
    /// Expectations evaluated in this specific order
    Sequence(ExpectationId, ExpectationId),
}

impl Expectation {
    pub fn new_group() -> Self {
        Expectation::Group(Vec::new())
    }

    pub fn new_call(name: MethodName) -> Self {
        Expectation::Call(name, Vec::new(), None)
    }

    pub fn verify(&mut self) -> ExpectationResult {
        unimplemented!()
    }

    // pub(crate) fn find_matches<I, O>(&self, matcher: &mut ExpectationMatcher<I, O>) where
    //     I: 'static,
    //     O: 'static
    // {
    //     match self {
    //         &Expectation::Group(ref vec) => {

    //         },
    //         &Expectation::Sequence(first, last) => {
    //             unimplemented!()
    //         },
    //         &Expectation::Call(name, ref vec, _) => {
    //             unimplemented!()
    //         },
    //         _ => { }
    //     }
    // }

    pub(crate) fn add_to_group(&mut self, id: ExpectationId) {
        match self {
            &mut Expectation::Group(ref mut vec) => {
                vec.push(id);
            },
            _ => panic!(".add_to_group() called on non-Group Expectation")
        }
    }

    pub(crate) fn add_to_call(&mut self, c_exp: CallExpectation) {
        match self {
            &mut Expectation::Call(_, ref mut vec, _) => {
                vec.push(c_exp);
            },
            _ => panic!(".add_to_call() called on non-Call Expectation")
        }
    }

    pub(crate) fn set_call_return(&mut self, return_behavior: Box<Any>) {
        match self {
            &mut Expectation::Call(_, _, ref mut ret_option) => {
                *ret_option = Some(return_behavior)
            },
            _ => panic!(".set_call_return() called on non-Call Expectation")
        }
    }
}

pub enum CallExpectation {
    /// A method must be called with arguments that meet certain requirements.
    /// The `Any` in the `Box` is a closure that can be downcasted later and called.
    Params(Box<Any>),
    /// A method must be called a certain number of times
    Times(i64),
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