use std::sync::Mutex;

pub type MethodName = &'static str;

pub struct MethodInner<I, O> {
    constraints: Vec<Box<Constraint<I>>>,
    name: MethodName,
    return_fn: Option<Box<FnMut(I) -> O>>
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

    fn was_called(&self, params: I) {
        // Tell constraints this was called
    }

    fn was_called_returning(&self, params: I) -> O {
        unimplemented!()
    }
}

pub trait Constraint<I> {
    #[allow(unused_variables)]
    fn handle_call(&mut self, params: I) { }

    /// At the end of the test, see if the Constraint passed or failed.
    fn verify(&self) -> Result<(), ()>;
}

trait CoolTrait {
    fn is_even(&self, num: i32) -> bool;
}

struct MockTrait {
    _is_even_m: Method<i32, bool>
}

impl MockTrait {
    fn new() -> Self {
        Self {
            _is_even_m: Method::new("is_even")
        }
    }

    fn expect_is_even(&mut self) -> &mut Method<i32, bool> {
        &mut self._is_even_m
    }
}

impl CoolTrait for MockTrait {
    fn is_even(&self, num: i32) -> bool {
        self._is_even_m.was_called_returning(num)
    }
}

fn main() {
    unimplemented!();
}