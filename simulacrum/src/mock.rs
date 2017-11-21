//! Mock object internals. Used by the macros to create Mocks for you, or you can
//! use this API to construct your own Mocks manually if you'd like!

use std::marker::PhantomData;
use std::any::Any;

use super::{ExpectationId, MethodName};
use super::expectation::{Expectation, ExpectationError, ExpectationResult};
use super::store::ExpectationsStore;
use super::user::{Method, MethodSig};

pub struct MethodData {
    calls_exact: Option<i64>,
    name: MethodName
}

// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
struct ExpectationMatcher<'a, I, O> {
    store: &'a Expectations,
    expectations: Vec<ExpectationId>,
    sig: MethodSig<I, O>
}

impl<'a, I, O> ExpectationMatcher<'a, I, O> {
    /// Validate params with param verifier closure the Mock user provided with `TrackedMethod.with()`.
    pub fn with(self, params: I) -> Self {
        // TODO: Validate params with param verifier fn
        unimplemented!()
    }

    /// Return the result of the closure the Mock user provided with `TrackedMethod.returning()`.
    pub fn returning(self) -> O {
        // TODO: Call returning behavior and return the result
        unimplemented!()
    }
}

pub struct Expectations {
    store: ExpectationsStore
}

impl Expectations {
    /// Create a new `Expectations` instance. Call this when your mock object is created,
    /// and store the `ExpectaionStore` object in it.
    pub fn new() -> Self {
        Expectations {
            store: ExpectationsStore::new()
        }
    }

    /// Returns a `Method` struct which you can use to add expectations for the method with the given name.
    pub fn expect<I, O>(&mut self, name: MethodName) -> Method<I, O> where
        I: 'static,
        O: 'static
    {
        Method::new(&mut self.store, name)
    }

    pub fn then(&mut self) {
        // TODO
        unimplemented!()
    }

    /// When a tracked method is called on the mock object, call this with the method's name
    /// in order to tell the `Expectations` that the method was called.
    pub fn was_called<I, O>(&self, name: MethodName, params: I) -> O where
        I: 'static,
        O: 'static
    {
        self.create_expectation_matcher(name)
            .with(params)
            .returning()
    }

    fn create_expectation_matcher<I, O>(&self, name: MethodName) -> ExpectationMatcher<I, O> where
        I: 'static,
        O: 'static
    {
        // self.store..get_mut(&name)
        //     .downcast::<ExpectationMatcher<I, O>>()
        //     .unwrap()
        unimplemented!()
    }

    /*
    fn is_tracked(&self, name: MethodName) -> bool {
        self.store.lock().unwrap().contains_key(name)
    }
    */

    fn verify(&self) {
        unimplemented!()
        // for (_, exp) in self.store.lock().unwrap().iter() {
        //     exp.verify();
        // }
    }
}

impl Drop for Expectations {
    /// All expectations will be verified when the mock object is dropped.
    fn drop(&mut self) {
        self.verify();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }