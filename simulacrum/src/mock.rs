//! Mock object internals. Use these to construct your own Mocks manually if you'd like!

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::interface::MethodSig;
use super::expectation::{Expectation, ExpectationError, ExpectationResult};


pub struct MethodData {
    calls_exact: Option<i64>,
    name: MethodName
}

pub trait ExpectationMatcherT { }

// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
pub struct ExpectationMatcher<'a, I, O> {
    store: &'a ExpectationStore,
    expectations: Vec<ExpectationId>,
    sig: MethodSig<I, O>
}

impl<'a, I, O> ExpectationMatcher<'a, I, O> {
    pub fn with(params: I) -> ExpectationResult {
        // TODO: Validate params with param verifier fn
        unimplemented!()
    }

    pub fn returning() -> O {
        // TODO: Call returning behavior and return the result
        unimplemented!()
    }
}

pub(crate) type ExpectationStoreInner = Mutex<HashMap<ExpectationId, Box<Expectation>>>;

pub struct ExpectationStore {
    inner: ExpectationStoreInner
}

impl ExpectationStore {
    /// Create a new `ExpectationStore` instance. Call this when your mock object is created,
    /// and store the `ExpectaionStore` object in it.
    pub fn new() -> Self {
        ExpectationStore {
            inner: Mutex::new(HashMap::new())
        }
    }

    /// When a tracked method is called on the mock object, call this with the method's key
    /// in order to tell the `ExpectationStore` that the method was called.
    pub fn was_called(&self, key: MethodName) -> Box<ExpectationMatcherT> {
        // TODO
        unimplemented!()

        // if self.is_tracked(&key) {
        //     self.inner.lock().unwrap().get_mut(&key).unwrap().was_called();
        // }
    }

    /*

    /// Signify that you'd like the `ExpectationStore` to track a method with the given key and name.
    ///
    /// Returns a `TrackedMethod` struct which you can use to add expectations for this particular method.
    pub fn track_method<'a>(&'a mut self, name: MethodName) -> TrackedMethod<'a> {
        TrackedMethod::new(&mut self.inner, name)
    }

    fn is_tracked(&self, name: MethodName) -> bool {
        self.inner.lock().unwrap().contains_key(name)
    }
    */

    fn verify(&self) {
        unimplemented!()
        // for (_, exp) in self.inner.lock().unwrap().iter() {
        //     exp.verify();
        // }
    }
}

impl Drop for ExpectationStore {
    /// All expectations will be verified when the mock object is dropped.
    fn drop(&mut self) {
        self.verify();
    }
}