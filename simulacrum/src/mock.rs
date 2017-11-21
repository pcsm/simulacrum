//! Mock object internals. Used by the macros to create Mocks for you, or you can
//! use this API to construct your own Mocks manually if you'd like!

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::any::Any;

use super::{ExpectationId, MethodName};
use super::interface::{Method, MethodSig};
use super::expectation::{Expectation, ExpectationError, ExpectationResult};


pub struct MethodData {
    calls_exact: Option<i64>,
    name: MethodName
}

// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
/// This is what is returned when calling `ExpectationsStore.was_called("METHOD_NAME")`.
/// Use its methods to flesh out the methods in your Mock.
pub struct ExpectationMatcher<'a, I, O> {
    store: &'a ExpectationStore,
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
    pub fn was_called<I, O>(&self, key: MethodName, params: I) -> O where
        I: 'static,
        O: 'static
    {
        self.was_called_internal("foo")
            .downcast::<ExpectationMatcher<I, O>>()
            .unwrap()
            .with(params)
            .returning()
    }

    fn was_called_internal(&self, key: MethodName) -> Box<Any> {
        // TODO
        unimplemented!()

        // if self.is_tracked(&key) {
        //     self.inner.lock().unwrap().get_mut(&key).unwrap().was_called();
        // }
    }

    /// Returns a `Method` struct which you can use to add expectations for the method with the given name.
    pub fn expect<I, O>(&mut self, name: MethodName) -> Method<I, O> where
        I: 'static,
        O: 'static
    {
        Method::new(&mut self.inner, name)
    }

    pub fn then(&mut self) {
        // TODO
        unimplemented!()
    }

    /*
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