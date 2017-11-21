//! This is the API that you'll call in your tests when using your Mock objects.

use std::any::Any;
use std::marker::PhantomData;

use super::{ExpectationId, MethodName};
use super::expectation::{CallExpectation, Expectation};
use super::store::ExpectationsStore;

// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
pub(crate) struct MethodSig<I, O> {
    pub(crate) input: PhantomData<I>,
    pub(crate) name: MethodName,
    pub(crate) output: PhantomData<O>
}

/// What you get from calling `.expect_METHOD_NAME()` on a Mock.
///
/// From here, use this struct's methods to set the number of calls expected.
#[must_use]
pub struct Method<'a, I, O> {
    store: &'a mut ExpectationsStore,
    sig: MethodSig<I, O>,
}

impl<'a, I, O> Method<'a, I, O> {
    pub(crate) fn new(store: &'a mut ExpectationsStore, name: MethodName) -> Self {
        let sig = MethodSig {
            input: PhantomData,
            name,
            output: PhantomData
        };
        Self {
            store,
            sig
        }
    }

    /// You expect this method to be called zero times.
    pub fn called_never(self) -> TrackedMethod<'a, I, O> {
        self.called_times(0)
    }

    /// You expect this method to be called only once.
    pub fn called_once(self) -> TrackedMethod<'a, I, O> {
        self.called_times(1)
    }

    /// You expect this method to be called `calls` number of times. 
    pub fn called_times(self, calls: i64) -> TrackedMethod<'a, I, O> {
        // Create an expectation that counts a certain number of calls.
        let mut exp = Expectation::new_call(self.sig.name);
        exp.add_to_call(CallExpectation::Times(calls));

        // Add the expectation to the store.
        let id = self.store.add(exp);

        TrackedMethod {
            id,
            method: self
        }
    }
}

/// Once you've specified the number of times you expect a method to be called,
/// you can specify additional behaviors and expectations through this object's
/// methods.
pub struct TrackedMethod<'a, I, O> {
    id: ExpectationId,
    method: Method<'a, I, O>
}

impl<'a, I, O> TrackedMethod<'a, I, O> {
    /// Specify a function that verifies the parameters.
    /// If it returns `false`, the expectation will be invalidated.
    pub fn with<F>(self, param_verifier: F) -> Self where
        F: 'static + FnMut(I) -> bool
    {
        let c_exp = CallExpectation::Params(Box::new(param_verifier));
        self.method.store.get_mut(self.id).add_to_call(c_exp);
        self
    }

    pub fn returning<F>(self, result_behavior: F) -> Self where
        F: 'static + FnMut(I) -> O
    {
        let b = Box::new(result_behavior);
        self.method.store.get_mut(self.id).set_call_return(b);
        self
    }
}