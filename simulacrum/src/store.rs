use handlebox::HandleBox;

use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::expectation::{Constraint, Expectation, ExpectationT, ExpectationResult};
use super::user::{MethodSig, MethodTypes};

// A thread-safe store for `Box<ExpectationT>`s, including the order that they should be
// evaluated in (Eras).
pub(crate) struct ExpectationStore(Mutex<Inner>);

struct Inner {
    current_unverified_era: usize,
    eras: Vec<Era>,
    expectations: HandleBox<Box<ExpectationT>>,
    status: ExpectationResult
}

type Era = Vec<ExpectationId>;

impl ExpectationStore {
    pub fn new() -> Self {
        let eras = vec![Era::new()];
        ExpectationStore(Mutex::new(Inner {
            current_unverified_era: 0,
            eras,
            expectations: HandleBox::new(),
            status: Ok(())
        }))
    }

    pub fn get_mut<I, O>(&self, id: ExpectationId) -> ExpectationEditor<I, O> {
        ExpectationEditor {
            id,
            store: &self,
            types: MethodTypes::new()
        }
    }

    pub fn matcher_for<I, O>(&self, name: MethodName) -> ExpectationMatcher<I, O> where
        I: 'static,
        O: 'static
    {
        let sig = MethodSig {
            name,
            types: MethodTypes::new()
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
    pub fn add<E>(&self, expectation: E) -> ExpectationId where
        E: ExpectationT + 'static
    {
        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();
        
        // Add a new expectation
        let id = inner.expectations.add(Box::new(expectation));

        // Add that new expectation to the current Era
        inner.eras.last_mut().unwrap().push(id);

        id
    }

    /// Verify all expectations in this store.
    pub fn verify(&self) -> ExpectationResult {
        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();

        // If all of our Eras are verfied, we're good to go!
        if inner.current_unverified_era >= inner.eras.len() {
            inner.status = Ok(());
            return inner.status.clone();
        }

        let mut current_unverified_era = inner.current_unverified_era;
        let mut status = Ok(());

        'eras: for era_index in inner.current_unverified_era .. inner.eras.len() {
            let era = &inner.eras[era_index];
            for id in era.iter() {
                let expectation = inner.expectations.get(id).unwrap();
                let r = expectation.verify();
                if r.is_err() {
                    // Note the current Era index
                    current_unverified_era = era_index;
                    // Note the error in this Era
                    status = r;
                    // Stop processing Eras since this one is still incomplete
                    break 'eras;
                }
            }
        }

        inner.current_unverified_era = current_unverified_era;
        inner.status = status;

        // Mark Eras as complete if all of their expectations have been met
        // 'eras: for era in inner.eras.iter() {
        //     // Once an era is complete, we don't need to check it anymore
        //     if !era.is_complete() {
        //         for id in era.expectations.iter() {
        //             let expectation = inner.expectations.get(id).unwrap();
        //             let r = expectation.verify();

        //             if r.is_err() {
        //                 // Note the error in this Era
        //                 era.status = r;
        //                 // Stop processing Eras since this one is still incomplete
        //                 break 'eras;
        //             }
        //         }

        //         // If we get here, it means that all the Expectations in this Era
        //         // have been met, so we can mark it as complete.
        //         era.status = Ok(());
        //     }
        // }

        // // Verify each Era in order
        // for era in inner.eras.iter_mut() {
        //     if !era.is_complete() {
        //         unimplemented!()
        //     }
        //     // if let Err(expectation_error) = era.status {
        //     // }
        // }

        Ok(())
    }

    fn status(&self) -> ExpectationResult {
        self.0.lock().unwrap().status.clone()
    }

    /// (For testing) Get the number of total Expectations in the store.
    fn exp_count(&self) -> usize {
        self.0.lock().unwrap().expectations.internal_map().len()
    }

    /// (For testing) Get the number of total Eras in the store.
    fn era_count(&self) -> usize {
        self.0.lock().unwrap().eras.len()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a, I, O> {
    id: ExpectationId,
    store: &'a ExpectationStore,
    types: MethodTypes<I, O>
}

impl<'a, I, O> ExpectationEditor<'a, I, O> where
    I: 'static,
    O: 'static
{
    pub(crate) fn constrain<C>(&self, constraint: C) where
        C: Constraint<I> + 'static
    {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().constrain(constraint);
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().set_return(return_behavior);
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
    store: &'a ExpectationStore
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
mod store_tests {
    use super::*;
    use constraint::ConstraintError;
    use constraint::stock::always::{AlwaysFail, AlwaysPass};

    #[test]
    fn test_new() {
        let s = ExpectationStore::new();

        assert!(s.status().is_ok(), "Store should be Ok after creation");
        assert_eq!(s.era_count(), 1, "Store should have one Era after creation");
        assert_eq!(s.exp_count(), 0, "Store should have no Expectations after creation");
    }

    #[test]
    fn test_add() {
        let s = ExpectationStore::new();
        let e: Expectation<(), ()> = Expectation::new("foo");

        s.add(e);

        assert_eq!(s.era_count(), 1, "Number of Eras");
        assert_eq!(s.exp_count(), 1, "Number of Expectations");
    }

    #[test]
    fn test_verify_no_expectations() {
        let s = ExpectationStore::new();

        let r = s.verify();

        assert!(r.is_ok());
    }

    #[test]
    fn test_verify_simple_pass() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("squig");
        e.constrain(AlwaysPass);
        s.add(e);

        let r = s.verify();

        assert!(r.is_ok(), "Store should pass");
    }

    #[test]
    fn test_verify_simple_fail() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("zooks");
        e.constrain(AlwaysFail);
        s.add(e);

        let r = s.verify();

        assert!(r.is_err(), "Store should fail");
        let r = r.unwrap_err();
        assert_eq!(r.method_name, "zooks", "Store error should have the correct method name");
        assert_eq!(r.constraint_err, ConstraintError::AlwaysFail, "Store error should contain the correct Constraint error");
    }

    #[test]
    fn test_verify_then() {
        // TODO
    }
}