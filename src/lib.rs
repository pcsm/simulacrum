use std::collections::HashMap;
use std::hash::Hash;

pub struct TrackedMethod {
    calls_exact: Option<i64>,
    name: String
}

impl TrackedMethod {
    fn new(name: String) -> Self {
        TrackedMethod {
            calls_exact: None,
            name 
        }
    }

    pub fn called_never(&mut self) {
        self.called_times(0);
    }

    pub fn called_once(&mut self) {
        self.called_times(1);
    }

    pub fn called_times(&mut self, calls: i64) {
        self.calls_exact = Some(calls);
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

pub struct ExpectationStore<K> where
    K: Eq + Hash
{
    inner: HashMap<K, TrackedMethod>
}

impl<K> ExpectationStore<K> where
    K: Eq + Hash
{
    /// Create a new `ExpectationStore` instance. Call this when your mock object is created,
    /// and store the `ExpectaionStore` object in it.
    pub fn new() -> Self {
        ExpectationStore {
            inner: HashMap::new()
        }
    }

    /// When a tracked method is called on the mock object, call this with the method's key
    /// in order to tell the `ExpectationStore` that the method was called.
    pub fn was_called(&mut self, key: K) {
        if self.is_tracked(&key) {
            self.inner.get_mut(&key).unwrap().was_called();
        }
    }

    /// Signify that you'd like the `ExpectationStore` to track a method with the given key and name.
    pub fn track_method<S: Into<String>>(&mut self, key: K, name: S) -> &mut TrackedMethod {
        self.inner.entry(key).or_insert_with(|| TrackedMethod::new(name.into()))
    }

    fn is_tracked(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    fn verify(&self) {
        for (_, exp) in self.inner.iter() {
            exp.verify();
        }
    }
}

impl<K> Drop for ExpectationStore<K> where
    K: Eq + Hash
{
    /// All expectations will be verified when the mock object is dropped.
    fn drop(&mut self) {
        self.verify();
    }
}
