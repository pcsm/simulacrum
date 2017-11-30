//! This is the API that you'll call in your tests when using your Mock objects.

use std::marker::PhantomData;

use super::{ExpectationId, MethodName};
use super::expectation::Expectation;
use super::constraint::stock::times::Times;
use super::constraint::stock::params::Params;
use super::store::ExpectationStore;

// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
pub(crate) struct MethodTypes<I, O> {
    pub(crate) input: PhantomData<I>,
    pub(crate) output: PhantomData<O>
}

impl<I, O> MethodTypes<I, O> {
    pub(crate) fn new() -> Self {
        MethodTypes {
            input: PhantomData,
            output: PhantomData
        }
    }
}

pub(crate) struct MethodSig<I, O> {
    pub(crate) name: MethodName,
    pub(crate) types: MethodTypes<I, O>
}

/// What you get from calling `.expect_METHOD_NAME()` on a Mock.
///
/// From here, use this struct's methods to set the number of calls expected.
#[must_use]
pub struct Method<'a, I, O> {
    store: &'a mut ExpectationStore,
    sig: MethodSig<I, O>,
}

impl<'a, I, O> Method<'a, I, O> where
    I: 'static,
    O: 'static
{
    pub(crate) fn new(store: &'a mut ExpectationStore, name: MethodName) -> Self {
        let types = MethodTypes {
            input: PhantomData,
            output: PhantomData
        };
        let sig = MethodSig {
            name,
            types
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
        let mut exp: Expectation<I, O> = Expectation::new(self.sig.name);
        exp.constrain(Times::new(calls));

        // Add the expectation to the store.
        let id = self.store.add(exp);

        TrackedMethod {
            id,
            method: self
        }
    }

    /// This method can be called any number of times, including zero.
    pub fn called_any(self) -> TrackedMethod<'a, I, O> {
        // Create an empty expectation
        let mut exp: Expectation<I, O> = Expectation::new(self.sig.name);

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

impl<'a, I, O> TrackedMethod<'a, I, O> where
    I: 'static,
    O: 'static
{
    /// Specify a function that verifies the parameters.
    /// If it returns `false`, the expectation will be invalidated.
    pub fn with<F>(self, param_verifier: F) -> Self where
        F: 'static + FnMut(I) -> bool
    {
        let constraint = Params::new(param_verifier);
        self.method.store.get_mut::<I, O>(self.id).constrain(constraint);
        self
    }

    pub fn returning<F>(self, result_behavior: F) -> Self where
        F: 'static + FnMut(I) -> O
    {
        self.method.store.get_mut::<I, O>(self.id).set_return(result_behavior);
        self
    }
}