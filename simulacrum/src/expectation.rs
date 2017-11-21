use std::fmt::Display;

use super::{ExpectationId, MethodName};

pub type ExpectationResult = Result<(), ExpectationError>;

pub enum ExpectationError {
    CalledTooFewTimes(MethodName, i64),
    CalledTooManyTimes(MethodName, i64),
    CalledOutOfOrder,
    MismatchedArgs(MethodName),
}

pub enum Expectation {
    /// Expectations that must all be met
    All(Vec<ExpectationId>),
    /// A method must be called
    Call(Vec<CallExpectation>),
    /// Expectations evaluated in this specific order
    Sequence(ExpectationId, ExpectationId),
}

impl Expectation {
    pub fn new_all() -> Self {
        Expectation::All(Vec::new())
    }

    pub fn validate(&mut self) -> ExpectationResult {
        unimplemented!()
    }

    pub(crate) fn add_to_all(&mut self, id: ExpectationId) {
        match self {
            &mut Expectation::All(ref mut vec) => {
                vec.push(id);
            },
            _ => panic!(".add_to_all() called on non-All Expectation")
        }
    }

    pub(crate) fn add_to_call(&mut self, c_exp: CallExpectation) {
        match self {
            &mut Expectation::Call(ref mut vec) => {
                vec.push(c_exp);
            },
            _ => panic!(".add_to_call() called on non-Call Expectation")
        }
    }
}

pub enum CallExpectation {
    /// A method must be called with arguments that meet certain requirements
    CallArgs,
    /// A method must be called a certain number of times
    CallTimes(i64),
}

/*

pub trait Expectation {
    fn validate(&mut self) -> ExpectationResult;
}

/// Expectation where all of the referred-to expectations must be valid for it to be valid.
pub struct All(Vec<ExpectationId>);

impl All {
    pub fn new() -> Self {
        All(Vec::new())
    }
}

impl Expectation for All {
    fn validate(&mut self) -> ExpectationResult {
        unimplemented!()
    }
}
*/

/*
use std::collections::vec_deque::VecDeque;

pub type TrackedMethodKey = &'static str;

trait CallArgsT {
    fn validate(&mut self) -> ExpectationResult;
}

pub enum ExpectationError {
    CalledTooFewTimes(TrackedMethodKey, i64),
    CalledTooManyTimes(TrackedMethodKey, i64),
    CalledOutOfOrder,
    MismatchedArgs(TrackedMethodKey),
}

// enum Expectation {
//     /// A method must be called with arguments that meet certain requirements
//     CallArgs(TrackedMethodKey, Box<CallArgsT>),
//     /// A method must be called a certain number of times
//     CallTimes(TrackedMethodKey, i64),
//     /// Expectations evaluated in any order
//     Group(Vec<Expectation>),
//     /// Expectations evaluated in this specific order
//     Sequence(VecDeque<Expectation>),
// }

struct CallTimes(TrackedMethodKey, i64);
impl Expectation for CallTimes {
    
}

struct Group(Vec<Box<Expectation>>);
impl Expectation for Group {
    fn validate(&mut self) -> ExpectationResult {
        for exp in self.0.iter() {
            exp.validate()?
        }
        Ok(())
    }
}

struct Sequence(VecDeque<Box<Expectation>>);
impl Expectation for Sequence {
    fn validate(&mut self) -> ExpectationResult {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ExpectationError::CalledOutOfOrder)
        }
    }
}

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