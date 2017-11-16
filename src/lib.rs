use std::collections::HashMap;
use std::hash::Hash;

pub struct TrackedMethod {
    calls_exact: Option<i64>,
    name: String
}

impl TrackedMethod {
    pub fn new(name: String) -> Self {
        TrackedMethod {
            calls_exact: None,
            name 
        }
    }

    pub fn called_times(&mut self, calls: i64) {
        self.calls_exact = Some(calls)
    }

    pub fn called(&mut self) {
        if let Some(calls) = self.calls_exact {
            self.calls_exact = Some(calls - 1)
        }
    }

    pub fn verify(&self) {
        match self.calls_exact {
            Some(x) if x < 0 => panic!("{} was called {} times more than expected", self.name, x.abs()),
            Some(x) if x > 0 => panic!("{} was called {} times fewer than expected", self.name, x),
            _ => { }
        }
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
    pub fn new() -> Self {
        ExpectationStore {
            inner: HashMap::new()
        }
    }

    pub fn verify(&self) {
        for (_, exp) in self.inner.iter() {
            exp.verify();
        }
    }

    pub fn called(&mut self, key: K) {
        if self.is_tracked(&key) {
            self.inner.get_mut(&key).unwrap().called();
        }
    }

    pub fn is_tracked(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn track_method<S: Into<String>>(&mut self, key: K, name: S) -> &mut TrackedMethod {
        self.inner.entry(key).or_insert_with(|| TrackedMethod::new(name.into()))
    }
}

// pub struct DrawBehaviorMock {
//     expectations: ExpectationStore<&'static str>
// }

// impl DrawBehaviorMock {
//     pub fn new() -> Self {
//         DrawBehaviorMock {
//             expectations: ExpectationStore::new()
//         }
//     }

//     pub fn expect(&mut self, name: &'static str) -> &mut TrackedMethod {
//         self.expectations.track_method(name, name)
//     }

//     pub fn expect_setup_draw(&mut self) -> &mut TrackedMethod {
//         self.expect("setup_draw")
//     }

//     pub fn expect_cleanup_draw(&mut self) -> &mut TrackedMethod {
//         self.expect("cleanup_draw")
//     }
// }

// impl Drop for DrawBehaviorMock {
//     fn drop(&mut self) {
//         self.expectations.verify();
//     }
// }

// impl<D> DrawBehavior<D> for DrawBehaviorMock {
//     fn draw(&mut self, _state: &ViewState<D>) {
//         self.expectations.called("draw");
//     }

//     fn setup_draw(&mut self, _state: &ViewState<D>) {
//         self.expectations.called("setup_draw");
//     }

//     fn cleanup_draw(&mut self, _state: &ViewState<D>) {
//         self.expectations.called("cleanup_draw");
//     }
// }