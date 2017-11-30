//! Mock object internals. Used by the macros to create Mocks for you, or you can
//! use this API to construct your own Mocks manually if you'd like!

use super::MethodName;
use super::store::ExpectationStore;
use super::user::Method;

pub struct Expectations {
    store: ExpectationStore
}

impl Expectations {
    /// Create a new `Expectations` instance. Call this when your mock object is created,
    /// and store the `ExpectaionStore` object in it.
    pub fn new() -> Self {
        Expectations {
            store: ExpectationStore::new()
        }
    }

    /// Returns a `Method` struct which you can use to add expectations for the method with the given name.
    pub fn expect<I, O>(&mut self, name: MethodName) -> Method<I, O> where
        I: 'static,
        O: 'static
    {
        Method::new(&mut self.store, name)
    }

    pub fn then(&mut self) -> &mut Self {
        self.store.new_era();
        self
    }

    /// When a tracked method is called on the mock object, call this with the method's name
    /// in order to tell the `Expectations` that the method was called.
    ///
    /// Does not return a value.
    pub fn was_called<I, O>(&self, name: MethodName, params: I) where
        I: 'static,
        O: 'static
    {
        self.store
            .matcher_for::<I, O>(name)
            .was_called(params);
    }

    /// Same as `was_called()`, but also returns the result.
    pub fn was_called_returning<I, O>(&self, name: MethodName, params: I) -> O where
        I: 'static,
        O: 'static
    {
        self.store
            .matcher_for::<I, O>(name)
            .was_called(params)
            .returning()
    }

    fn verify(&self) {
        if let Err(e) = self.store.verify() {
            panic!("Unmet Expectations: {}", e);
        }
    }
}

impl Drop for Expectations {
    /// All expectations will be verified when the mock object is dropped.
    fn drop(&mut self) {
        self.verify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_called_once() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("spoo").called_once();
        
        e.was_called::<(), ()>("spoo", ());

        // Verified on drop
    }
}