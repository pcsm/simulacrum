extern crate handlebox;

pub mod expectation;
pub mod user;
pub mod mock;
mod store;

pub type MethodName = &'static str;

pub use handlebox::Handle as ExpectationId;

pub use self::mock::Expectations;
pub use self::user::Method;

/*
impl TrackedMethodData {
    fn new(name: TrackedMethodKey) -> Self {
        Self {
            calls_exact: None,
            name 
        }
    }

    fn was_called(&mut self) {
        if let Some(calls) = self.calls_exact {
            self.calls_exact = Some(calls - 1);
        }
    }

    fn verify(&self) {
        match self.calls_exact {
            Some(x) if x < 0 => panic!("{} was called {} times more than expected", self.name, x.abs()),
            Some(x) if x > 0 => panic!("{} was called {} times fewer than expected", self.name, x),
            _ => { }
        };
    }
}

macro_rules! get_tracked_method {
    ($target:ident, $name:ident) => {
        $target.inner.lock().unwrap().entry($name).or_insert_with(|| TrackedMethodData::new($name))
    }
}

struct MethodReturn<T> {
    reaction: Box<FnMut() -> T>
}

// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
pub struct TrackedMethod<'a, I, O> {
    inner: &'a mut ExpectationStore,
    name: TrackedMethodKey
}

impl<'a, I, O> TrackedMethod<'a, I, O> {
    fn new(inner: &'a mut ExpectationStore, name: TrackedMethodKey) -> Self {
        TrackedMethod {
            inner,
            name
        }
    }

    /// You expect this method to be called zero times.
    pub fn called_never(&mut self) {
        self.called_times(0);
    }

    /// You expect this method to be called only once.
    pub fn called_once(&mut self) {
        self.called_times(1);
    }

    /// You expect this method to be called `calls` number of times. 
    pub fn called_times(&mut self, calls: i64) {
        let name = self.name;
        get_tracked_method!(self, name).calls_exact = Some(calls);
    }

    pub fn with(&mut self, args: I) {
        // TODO
        let name = self.name;
        get_tracked_method!(self, name).calls_exact = Some(calls);
    }

    pub fn returning<F>(&mut self, closure: F) where
        F: FnMut() -> O
    {
        // TODO
    }
}
*/