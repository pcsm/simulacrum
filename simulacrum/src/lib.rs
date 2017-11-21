use std::collections::HashMap;
use std::sync::Mutex;
use std::marker::PhantomData;

pub mod expectation;
pub mod interface;

pub type MethodName = &'static str;

pub type ExpectationId = usize;

pub struct TrackedMethodData {
    calls_exact: Option<i64>,
    name: MethodName
}

type ExpectationStoreInner = Mutex<HashMap<MethodName, TrackedMethodData>>;

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
    inner: &'a mut ExpectationStoreInner,
    name: TrackedMethodKey
}

impl<'a, I, O> TrackedMethod<'a, I, O> {
    fn new(inner: &'a mut ExpectationStoreInner, name: TrackedMethodKey) -> Self {
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

pub struct ExpectationStore {
    inner: ExpectationStoreInner
}

impl ExpectationStore {
    /// Create a new `ExpectationStore` instance. Call this when your mock object is created,
    /// and store the `ExpectaionStore` object in it.
    pub fn new() -> Self {
        ExpectationStore {
            inner: Mutex::new(HashMap::new())
        }
    }

    /// When a tracked method is called on the mock object, call this with the method's key
    /// in order to tell the `ExpectationStore` that the method was called.
    pub fn was_called(&self, key: TrackedMethodKey) {
        if self.is_tracked(&key) {
            self.inner.lock().unwrap().get_mut(&key).unwrap().was_called();
        }
    }

    /// Signify that you'd like the `ExpectationStore` to track a method with the given key and name.
    ///
    /// Returns a `TrackedMethod` struct which you can use to add expectations for this particular method.
    pub fn track_method<'a>(&'a mut self, name: TrackedMethodKey) -> TrackedMethod<'a> {
        TrackedMethod::new(&mut self.inner, name)
    }

    fn is_tracked(&self, name: TrackedMethodKey) -> bool {
        self.inner.lock().unwrap().contains_key(name)
    }

    fn verify(&self) {
        for (_, exp) in self.inner.lock().unwrap().iter() {
            exp.verify();
        }
    }
}

impl Drop for ExpectationStore {
    /// All expectations will be verified when the mock object is dropped.
    fn drop(&mut self) {
        self.verify();
    }
}
*/