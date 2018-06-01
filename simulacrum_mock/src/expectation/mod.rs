use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use super::MethodName;

pub mod result;

pub use super::constraint::Constraint;
pub use self::result::{ExpectationError, ExpectationResult};

/// An expectation that a method must be called. Also includes an optional
/// closure to produce return values, if necessary.
pub struct Expectation<I, O> where
    I: 'static
{
    name: MethodName,
    constraints: Vec<Box<Constraint<I>>>,
    modification_fn: Option<Box<FnMut(&mut I)>>,
    return_fn: Option<Box<FnMut(I) -> O>>
}

impl<I, O> Expectation<I, O> where
    I: 'static,
    O: 'static
{
    pub fn new(name: MethodName) -> Self {
        Expectation {
            name,
            constraints: Vec::new(),
            modification_fn: None,
            return_fn: None
        }
    }

    pub fn handle_call(&mut self, params_cell: &RefCell<I>) {
        self.constraints_handle_call(params_cell);
        self.run_modification_behavior(params_cell);
    }

    fn constraints_handle_call(&mut self, params_cell: &RefCell<I>) {
        for constraint in self.constraints.iter_mut() {
            let params = params_cell.borrow();
            constraint.handle_call(params.deref());
        }
    }

    fn run_modification_behavior(&mut self, params_cell: &RefCell<I>) {
        if self.modification_fn.is_some() {
            let mut params = params_cell.borrow_mut();
            (self.modification_fn.as_mut().unwrap())(params.deref_mut())
        }
    }

    pub fn return_value_for(&mut self, params_cell: RefCell<I>) -> O {
        if self.return_fn.is_some() {
            (self.return_fn.as_mut().unwrap())(params_cell.into_inner())
        } else {
            panic!("No return closure specified for `{}`, which should return.", self.name);
        }
    }

    pub(crate) fn constrain<C>(&mut self, constraint: C) where
        C: Constraint<I> + 'static
    {
        self.constraints.push(Box::new(constraint));
    }

    pub(crate) fn set_modification<F>(&mut self, modification_behavior: F) where
        F: 'static + FnMut(&mut I)
    {
        self.modification_fn = Some(Box::new(modification_behavior));
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.return_fn = Some(Box::new(return_behavior));
    }
}

pub trait ExpectationT {
    fn as_any(&mut self) -> &mut Any;

    fn verify(&self) -> ExpectationResult;

    fn name(&self) -> MethodName;
}

impl<I, O> ExpectationT for Expectation<I, O> where
    I: 'static,
    O: 'static
{
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn verify(&self) -> ExpectationResult {
        for constraint in self.constraints.iter() {
            if let Err(constraint_err) = constraint.verify() {
                return Err(ExpectationError {
                    constraint_err,
                    method_name: self.name
                })
            }
        }
        Ok(())
    }

    fn name(&self) -> MethodName {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constraint::{ConstraintError, ConstraintMock};
    use constraint::stock::always::{AlwaysFail, AlwaysPass};
    use std::{sync::Arc, cell::RefCell};

    #[test]
    fn test_new() {
        let e: Expectation<(), ()> = Expectation::new("foo");

        assert_eq!(e.name, "foo", "Name of Constraint should be `foo`");
        assert_eq!(e.constraints.len(), 0, "Number of Constraints should be 0");
        assert!(e.return_fn.is_none(), "Return Behavior Should Not Exist");
        assert!(e.modification_fn.is_none(), "Modification Behavior Should Not Exist");
    }

    #[test]
    fn test_handle_call() {
        let mut e: Expectation<(), ()> = Expectation::new("foo");
        let mut m = ConstraintMock::new();
        m.expect_handle_call();
        e.constrain(m);

        e.handle_call(&RefCell::new(()));

        // ConstraintMock verifies on Drop
    }

    #[test]
    fn test_constrain() {
        let mut e: Expectation<(), ()> = Expectation::new("test");

        e.constrain(AlwaysPass);

        assert_eq!(e.constraints.len(), 1, "Number of Constraints should be 1");
    }

    #[test]
    fn test_return() {
        let mut e: Expectation<(), i32> = Expectation::new("yaz");

        e.set_return(|_| 5);

        assert!(e.return_fn.is_some(), "Return Closure Should Exist");
        assert_eq!(e.return_value_for(RefCell::new(())), 5, "Return Closure Should return 5");
    }

    #[test]
    fn test_return_consuming() {
        // Does not implement Clone or Copy
        struct UniquelyOwned(u32);

        let mut e: Expectation<UniquelyOwned, ()> = Expectation::new("foo");

        let dest: Arc<RefCell<Option<UniquelyOwned>>> =
            Arc::new(RefCell::new(None));
        let dest2 = dest.clone();
        e.set_return(move |x| {
            dest2.replace(Some(x));
        });
        e.return_value_for(RefCell::new(UniquelyOwned(42)));

        assert!(dest.borrow().is_some());
    }

    #[test]
    fn test_set_modification() {
        let mut e: Expectation<(), ()> = Expectation::new("bing");

        e.set_modification(|_| ());

        assert!(e.modification_fn.is_some(), "Modification Closure Should Exist");
    }

    #[test]
    #[should_panic]
    fn test_return_no_closure_given() {
        let mut e: Expectation<(), i32> = Expectation::new("yaz");

        // Did not set the return here

        // Panic: .returning() was not called, so we don't know what to return
        e.return_value_for(RefCell::new(()));
    }

    #[test]
    fn test_verify_pass() {
        let mut e: Expectation<(), ()> = Expectation::new("zonk");

        e.constrain(AlwaysPass);
        e.constrain(AlwaysPass);
        let r = e.verify();

        assert!(r.is_ok(), "Expectation should pass");
    }

    #[test]
    fn test_verify_fail() {
        let mut e: Expectation<(), ()> = Expectation::new("boop");

        e.constrain(AlwaysPass);
        e.constrain(AlwaysFail); // Will cause Expectation to fail
        let r = e.verify();

        assert!(r.is_err(), "Expectation should fail");
        let r = r.unwrap_err();
        assert_eq!(r.method_name, "boop", "Expectation error should have the correct method name");
        assert_eq!(r.constraint_err, ConstraintError::AlwaysFail, "Expectation error should contain the correct Constraint error");
    }
}
