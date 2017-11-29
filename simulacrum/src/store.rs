use handlebox::HandleBox;

use std::any::Any;
use std::marker::PhantomData;
use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::expectation::{CallExpectation, Expectation, ExpectationEra, ExpectationResult};
use super::user::MethodSig;

// A thread-safe store for Expectations, including the order that they should be
// evaluated in (Eras).
pub(crate) struct ExpectationsStore(Mutex<Inner>);

struct Inner {
    eras: Vec<ExpectationEra>,
    expectations: HandleBox<Expectation>
}

impl ExpectationsStore {
    pub fn new() -> Self {
        let eras = vec![ExpectationEra::new()];
        ExpectationsStore(Mutex::new(Inner {
            eras,
            expectations: HandleBox::new()
        }))
    }

    pub fn get_mut(&self, id: ExpectationId) -> ExpectationEditor {
        ExpectationEditor {
            id,
            store: &self
        }
    }

    pub fn matcher_for<I, O>(&self, name: MethodName) -> ExpectationMatcher<I, O> where
        I: 'static,
        O: 'static
    {
        let sig = MethodSig {
            input: PhantomData,
            name,
            output: PhantomData
        };
        let mut matcher = ExpectationMatcher {
            ids: Vec::new(),
            sig,
            store: &self
        };

        // Gather up ids for expectations that match this one
        // self.get_mut(self.top_group).find_matches()
        // for (id, exp) in self.expectations.lock().unwrap().internal_map().iter() {

        // }
        matcher
    }

    // Add a new Expectation under the current Era and return its id.
    pub fn add(&mut self, expectation: Expectation) -> ExpectationId {
        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();
        
        // Add a new expectation
        let id = inner.expectations.add(expectation);

        // Add that new expectation to the current Era
        inner.eras.last_mut().unwrap().add(id);

        id
    }

    // Verify all expectations in this store.
    pub fn verify(&self) -> ExpectationResult {
        unimplemented!()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a> {
    id: ExpectationId,
    store: &'a ExpectationsStore
}

impl<'a> ExpectationEditor<'a> {
    pub(crate) fn add(&self, c_exp: CallExpectation) {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().add(c_exp);
    }

    pub(crate) fn set_return(&mut self, return_behavior: Box<Any>) {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().set_return(return_behavior);
    }

    fn verify(&self) -> ExpectationResult {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().verify()
    }
}

// Used internally to mutably access an `ExpectationStore` when trying to apply
// a method to a set of matched expectations.
//
// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
pub(crate) struct ExpectationMatcher<'a, I, O> {
    ids: Vec<ExpectationId>,
    sig: MethodSig<I, O>,
    store: &'a ExpectationsStore
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}