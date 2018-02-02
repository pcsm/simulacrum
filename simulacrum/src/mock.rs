//! Mock object internals. You can use this API to construct mock objects manually.

use std::thread;

use super::MethodName;
use super::store::ExpectationStore;
use super::user::Method;

pub struct Expectations {
    store: ExpectationStore
}

impl Expectations {
    /// Create a new `Expectations` instance. Call this when your mock object is created.
    pub fn new() -> Self {
        Expectations {
            store: ExpectationStore::new()
        }
    }

    /// Returns a `Method` struct which you can use to add expectations for the 
    /// method with the given name.
    pub fn expect<I, O>(&mut self, name: MethodName) -> Method<I, O> where
        I: 'static,
        O: 'static
    {
        Method::new(&mut self.store, name)
    }

    /// Begin a new Era. Expectations in one Era must be met before expectations 
    /// in future eras will be evaluated.
    /// 
    /// Note that Eras are evaluated eagerly. This means that Eras may advance more
    /// quickly than you'd intuitively expect in certain situations. For example, 
    /// `called_any()` is marked as complete after the first call is received.
    /// This menas that, for the purposes of telling if an Era should be advanced or
    /// not, `called_any()` and `called_once()` are the same.
    pub fn then(&mut self) -> &mut Self {
        self.store.new_era();
        self
    }

    /// When a tracked method is called on the mock object, call this with the method's name
    /// in order to tell the `Expectations` that the method was called.
    ///
    /// Unlike `was_called_returning`, this method does not return a value.
    pub fn was_called<I, O>(&self, name: MethodName, params: I) where
        I: 'static,
        O: 'static
    {
        self.store
            .matcher_for::<I, O>(name)
            .was_called(params);
    }

    /// Same as the `was_called` method, but also returns the result.
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
            panic!("{}", e);
        }
    }
}

impl Drop for Expectations {
    /// All expectations will be verified when the mock object is dropped, 
    /// panicking if any of them are unmet.
    ///
    /// In the case where the Expectations object is being dropped because the
    /// thread is _already_ panicking, the Expectations object is not verified.
    fn drop(&mut self) {
        if !thread::panicking() {
            self.verify();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;
    use validator::stock::compare::*;

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

        // Panic: "spoo" should have been called once, but was never called
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
        
        // Panic: "nom" should have been called twice, but was only called once
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
        
        // Panic: "blitz" should have never been called
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
        e.expect::<i32, ()>("doog").called_once().with(gt(5));
        
        e.was_called::<i32, ()>("doog", 10);
    }

    #[test]
    #[should_panic]
    fn test_param_fail() {
        let mut e = Expectations::new();
        e.expect::<i32, ()>("doog").called_once().with(gt(5));
        
        // Panic: "doog"'s parameter was not > 5
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

    #[test]
    fn test_modifications() {
        let mut e = Expectations::new();
        e.expect::<*mut i32, ()>("dawg")
         .called_any()
         .modifying(|&mut arg| {
            unsafe {
                *arg = 3;
            } 
         });

        let mut i = 2;
        e.was_called::<*mut i32, ()>("dawg", &mut i);

        assert_eq!(i, 3);
    }

    #[test]
    fn test_then() {
        let mut e = Expectations::new();
        e.expect::<i32, ()>("fren").called_once().with(gt(5));
        e.then().expect::<i32, ()>("fren").called_once().with(lt(3));
        
        e.was_called::<i32, ()>("fren", 10); // Matches first era, completing it
        e.was_called::<i32, ()>("fren", 1); // Matches second era, completing it
    }

    #[test]
    fn test_then_never() {
        // Test to see that `called_never()` is only enforced until the era is completed
        let mut e = Expectations::new();
        e.expect::<(), ()>("bruh").called_never();
        e.expect::<(), ()>("fren").called_once();
        e.then().expect::<(), ()>("bruh").called_once();
        
        e.was_called::<(), ()>("fren", ()); // Matches first era, completing it
        e.was_called::<(), ()>("bruh", ()); // Matches second era, completing it
    }

    #[test]
    #[should_panic]
    fn test_then_partial_fail() {
        let mut e = Expectations::new();
        e.expect::<i32, ()>("fren").called_once().with(gt(5));
        e.then().expect::<i32, ()>("fren").called_times(2).with(lt(3));
        
        e.was_called::<i32, ()>("fren", 10); // Matches first era, completing it
        e.was_called::<i32, ()>("fren", 1); // Matches second era, but still incomplete

        // Panic: "fren" was expected to be called twice in the second era
    }

    #[test]
    fn test_then_multi_call() {
        let mut e = Expectations::new();

        // These expectations can be called in any order
        e.expect::<(), ()>("eh").called_once();
        e.expect::<(), ()>("donk").called_times(2); 

        // These expectations are called afterwards in any order
        e.then().expect::<(), ()>("calxx").called_times(3);
        e.expect::<(), ()>("mer").called_once();
        
        e.was_called::<(), ()>("donk", ());
        e.was_called::<(), ()>("eh", ());
        e.was_called::<(), ()>("donk", ()); // Completes first era
        e.was_called::<(), ()>("calxx", ());
        e.was_called::<(), ()>("mer", ());
        e.was_called::<(), ()>("calxx", ());
        e.was_called::<(), ()>("calxx", ()); // Completes second era
    }

    #[test]
    #[should_panic]
    fn test_then_wrong_order_fail() {
        let mut e = Expectations::new();

        // These expectations can be called in any order
        e.expect::<(), ()>("eh").called_once();
        e.expect::<(), ()>("donk").called_once();

        // These expectations are called afterwards in any order
        e.then().expect::<(), ()>("calxx").called_once();
        e.expect::<(), ()>("mer").called_once();
        
        e.was_called::<(), ()>("mer", ()); // No matching expectations
        e.was_called::<(), ()>("calxx", ()); // No matching expectations
        e.was_called::<(), ()>("donk", ()); 
        e.was_called::<(), ()>("eh", ()); // Completes first era

        // Panic: Second era was never completed
    }

    #[test]
    fn test_then_specific() {
        let mut e = Expectations::new();

        e.expect::<(), ()>("eh").called_once();
        e.then().expect::<(), ()>("donk").called_once();
        e.then().expect::<(), ()>("mer").called_once();
        e.expect::<(), ()>("eh").called_once();

        e.was_called::<(), ()>("eh", ()); // Completes first era
        e.was_called::<(), ()>("donk", ()); // Completes second era
        e.was_called::<(), ()>("eh", ());
        e.was_called::<(), ()>("mer", ()); // Completes third era
    }

    #[test]
    fn test_empty_era_leading() {
        let mut e = Expectations::new();

        // Create an empty era at the start
        e.then();
        e.expect::<(), ()>("eh").called_once();

        e.was_called::<(), ()>("eh", ()); // Completes first and second eras
    }

    #[test]
    fn test_empty_era_middle() {
        let mut e = Expectations::new();

        // Create an empty era in the middle
        e.expect::<(), ()>("eh").called_once();
        e.then();
        e.then();
        e.expect::<(), ()>("eh").called_once();

        e.was_called::<(), ()>("eh", ()); // Completes first and second eras
        e.was_called::<(), ()>("eh", ()); // Completes third era
    }

    #[test]
    fn test_empty_era_end() {
        let mut e = Expectations::new();

        e.expect::<(), ()>("eh").called_once();
        // Create an empty era at the end
        e.then();

        e.was_called::<(), ()>("eh", ()); // Completes first and second eras
    }

    #[test]
    fn test_one_expectation_per_era() {
        let mut e = Expectations::new();

        e.expect::<(), ()>("eh").called_never();
        let result = panic::catch_unwind(
            panic::AssertUnwindSafe(|| {
                e.expect::<(), ()>("eh").called_never(); // Panic: only one expectation should be registered for "eh" in a given era
            })
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_eras_complete_eagerly() {
        let mut e = Expectations::new();

        // Expectations
        e.expect::<(), ()>("c").called_any();
        e.then();
        e.expect::<(), ()>("d").called_once();
        
        // Calls
        e.was_called::<(), ()>("c", ()); // Completes first era
        e.was_called::<(), ()>("d", ()); // Completes second era
    }
}