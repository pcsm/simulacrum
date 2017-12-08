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

pub struct Method<'a, I, O>(Mutex<MethodInner<'a, I, O>>);

impl<'a, I, O> Method<'a, I, O> {
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

    pub fn returning<F>(&self, result_behavior: F) -> &Self where
        F: 'a,
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

/* DEMO */

trait CoolTrait {
    fn is_even(&self, num: i32) -> bool;
    fn lowercase(&self, s: &mut str);
    fn multiply(&self, result: &mut i32, factor: i32);
}

struct CoolTraitMock<'a> {
    m_is_even: Method<'a, i32, bool>,
    m_lowercase: Method<'a, *mut str, ()>,
    m_multiply: Method<'a, (*mut i32, i32), ()>
}

impl<'a> CoolTraitMock<'a> {
    fn new() -> Self {
        Self {
            m_is_even: Method::new("is_even"),
            m_lowercase: Method::new("lowercase"),
            m_multiply: Method::new("multiply")
        }
    }

    fn expect_is_even(&mut self) -> &mut Method<'a, i32, bool> {
        &mut self.m_is_even
    }

    fn expect_lowercase(&mut self) -> &mut Method<'a, *mut str, ()> {
        &mut self.m_lowercase
    }

    fn expect_multiply(&mut self) -> &mut Method<'a, (*mut i32, i32), ()> {
        &mut self.m_multiply
    }
}

impl<'a> CoolTrait for CoolTraitMock<'a> {
    fn is_even(&self, num: i32) -> bool {
        self.m_is_even.was_called_returning(num)
    }

    fn lowercase(&self, s: &mut str) {
        // self.m_lowercase.was_called_returning(s)
        self.m_lowercase.was_called_returning(s as *mut str)
    }

    fn multiply(&self, result: &mut i32, factor: i32) {
        self.m_multiply.was_called_returning((result as *mut i32, factor))
    }
}

fn main() {
    let mut m = CoolTraitMock::new(); 
    m.expect_is_even();
    m.expect_lowercase().returning(|s| {
        unsafe {
            s.as_mut().unwrap().make_ascii_lowercase();
        }
    });
    m.expect_multiply().returning(|args| {
        unsafe {
            *args.0 = 10;
        }
    });

    let mut s = "HELLO".to_owned();
    m.lowercase(&mut s);
    assert_eq!(s, "hello".to_owned());

    let mut res = 5;
    m.multiply(&mut res, 999);
    assert_eq!(res, 10);
}