use handlebox::HandleBox;

use std::any::Any;
use std::marker::PhantomData;
use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::expectation::{CallExpectation, Expectation, ExpectationResult};
use super::user::MethodSig;

// A HandleBox of Expectations, with one of them being a top-level `Group` that every
// other Expectation is a member of.
pub(crate) struct ExpectationsStore {
    exps: Mutex<HandleBox<Expectation>>,
    top_group: ExpectationId
}

impl ExpectationsStore {
    pub fn new() -> Self {
        let mut hb = HandleBox::new();
        let top_group = hb.add(Expectation::new_group());
        ExpectationsStore {
            exps: Mutex::new(hb),
            top_group
        }
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
        // for (id, exp) in self.exps.lock().unwrap().internal_map().iter() {

        // }
        matcher
    }

    // Add a new Expectation under the top-level `Group` and return its id.
    pub fn add(&mut self, expectation: Expectation) -> ExpectationId {
        let id = self.exps.lock().unwrap().add(expectation);
        self.get_mut(self.top_group).add_to_group(id);
        id
    }

    // Verify all expectations in this store.
    pub fn verify(&self) -> ExpectationResult {
        self.get_mut(self.top_group).verify()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a> {
    id: ExpectationId,
    store: &'a ExpectationsStore
}

impl<'a> ExpectationEditor<'a> {
    fn add_to_group(&self, id: ExpectationId) {
        self.store.exps.lock().unwrap().get_mut(&self.id).unwrap().add_to_group(id);
    }

    pub(crate) fn add_to_call(&self, c_exp: CallExpectation) {
        self.store.exps.lock().unwrap().get_mut(&self.id).unwrap().add_to_call(c_exp);
    }

    pub(crate) fn set_call_return(&mut self, return_behavior: Box<Any>) {
        self.store.exps.lock().unwrap().get_mut(&self.id).unwrap().set_call_return(return_behavior);
    }

    fn verify(&self) -> ExpectationResult {
        self.store.exps.lock().unwrap().get_mut(&self.id).unwrap().verify()
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }