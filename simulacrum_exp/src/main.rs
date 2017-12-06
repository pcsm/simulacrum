use std::sync::Mutex;

pub type MethodName = &'static str;

pub struct MethodInner<'a, I, O> {
    constraints: Vec<Box<Constraint<I>>>,
    name: MethodName,
    return_fn: Option<Box<'a + FnMut(I) -> O>>
}

impl<'a, I, O> MethodInner<'a, I, O> {
    fn was_called(&mut self, params: I) {
        self.constraints_handle_call(&params)
    }

    fn constraints_handle_call(&mut self, params: &I) {
        for constraint in self.constraints.iter_mut() {
            constraint.handle_call(&params);
        }
    }

    fn was_called_returning(&mut self, params: I) -> O {
        self.constraints_handle_call(&params);
        self.return_value_for(params)
    }

    fn return_value_for(&mut self, params: I) -> O {
        if self.return_fn.is_some() {
            (self.return_fn.as_mut().unwrap())(params)
        } else {
            panic!("No return closure specified for `{}`, which should return.", self.name);
        }
    }
}

pub struct Method<I, O>(Mutex<MethodInner<I, O>>);

impl<I, O> Method<I, O> {
    fn new(name: MethodName) -> Self {
        Method(Mutex::new(MethodInner {
            constraints: Vec::new(),
            name,
            return_fn: None
        }))
    }

    // MOCK

    fn was_called(&self, params: I) {
        self.0.lock().unwrap().was_called(params)
    }

    fn was_called_returning(&self, params: I) -> O {
        self.0.lock().unwrap().was_called_returning(params)
    }

    // USER

    // /// You expect this method to be called `calls` number of times. 
    // pub fn called_times(self, calls: i64) -> Self {
    //     // Create an expectation that counts a certain number of calls.
    //     let mut exp: Expectation<I, O> = Expectation::new(self.sig.name);
    //     exp.constrain(Times::new(calls));

    //     // Add the expectation to the store.
    //     let id = self.store.add(exp);

    //     method: self
    // }

    /// Specify a function that verifies the parameters.
    /// If it returns `false`, the expectation will be invalidated.
    // pub fn with<F>(self, param_verifier: F) -> Self where
    //     F: FnMut(&I) -> bool
    // {
    //     let constraint = Params::new(param_verifier);
    //     self.method.store.get_mut::<I, O>(self.id).constrain(constraint);
    //     self
    // }

    pub fn returning<F>(self, result_behavior: F) -> Self where
        F: FnMut(I) -> O
    {
        self.0.lock().unwrap().return_fn = Some(Box::new(result_behavior));
        self
    }
}

pub trait Constraint<I> {
    #[allow(unused_variables)]
    fn handle_call(&mut self, params: &I) { }

    /// At the end of the test, see if the Constraint passed or failed.
    fn verify(&self) -> Result<(), ()>;
}

trait CoolTrait {
    fn is_even(&self, num: i32) -> bool;
}

/* DEMO */

struct CoolTraitMock {
    _is_even_m: Method<i32, bool>
}

impl CoolTraitMock {
    fn new() -> Self {
        Self {
            _is_even_m: Method::new("is_even")
        }
    }

    fn expect_is_even(&mut self) -> &mut Method<i32, bool> {
        &mut self._is_even_m
    }
}

impl CoolTrait for CoolTraitMock {
    fn is_even(&self, num: i32) -> bool {
        self._is_even_m.was_called_returning(num)
    }
}

fn main() {
    let mut m = CoolTraitMock::new(); 
    m.expect_is_even();
}