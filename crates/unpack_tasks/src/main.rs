use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::task_local;
#[derive(Debug, Clone)]
struct FibCache {
    cache: Arc<Mutex<HashMap<u64, u64>>>,
}
impl FibCache {
    pub fn new() -> Self {
        FibCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get(&self, n: u64) -> Option<u64> {
        let cache = self.cache.lock().unwrap();
        cache.get(&n).cloned()
    }
    pub fn update(&self, n: u64, value: u64) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(n, value);
    }
}

task_local! {
    static CACHE: FibCache
}
fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let cache = CACHE.with(|c| c.clone());
    if let Some(result) = cache.get(n) {
        return result;
    }

    let result = fib(n - 1) + fib(n - 2);

    cache.update(n, result);

    result
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    CACHE.sync_scope(FibCache::new(), || {
        let result = fib(50);
        println!("Fibonacci of 10 is: {}", result);
    });
}
