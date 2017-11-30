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
            .was_called_returning(params)
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

    #[test]
    #[should_panic]
    fn test_called_once_fail() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("spoo").called_once();
    }

    #[test]
    fn test_called_twice() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("nom").called_times(2);
        
        e.was_called::<(), ()>("nom", ());
        e.was_called::<(), ()>("nom", ());
    }

    #[test]
    #[should_panic]
    fn test_called_twice_fail() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("nom").called_times(2);
        
        e.was_called::<(), ()>("nom", ());
    }

    #[test]
    fn test_called_never_pass() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("blitz").called_never();
    }

    #[test]
    #[should_panic]
    fn test_called_never_fail() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("blitz").called_never();
        
        e.was_called::<(), ()>("blitz", ());
    }

    #[test]
    fn test_called_any_zero() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("mega").called_any();
    }

    #[test]
    fn test_called_any_two() {
        let mut e = Expectations::new();
        e.expect::<(), ()>("mega").called_any();

        e.was_called::<(), ()>("mega", ());
        e.was_called::<(), ()>("mega", ());
    }

    #[test]
    fn test_param() {
        let mut e = Expectations::new();
        e.expect::<i32, ()>("doog").called_once().with(|&arg| arg > 5);
        
        e.was_called::<i32, ()>("doog", 10);
    }

    #[test]
    #[should_panic]
    fn test_param_fail() {
        let mut e = Expectations::new();
        e.expect::<i32, ()>("doog").called_once().with(|&arg| arg > 5);
        
        e.was_called::<i32, ()>("doog", 1);
    }

    #[test]
    fn test_returning() {
        let mut e = Expectations::new();
        e.expect::<(), i32>("boye").called_any().returning(|_| 5);

        let r = e.was_called_returning::<(), i32>("boye", ());

        assert_eq!(r, 5);
    }

    #[test]
    #[should_panic]
    fn test_returning_no_matches() {
        let e = Expectations::new();

        // Not expecting "boye" here, so when it's called, we should panic

        // Panic: No expectation matches, so we can't return a value
        e.was_called_returning::<(), i32>("boye", ());
    }
}