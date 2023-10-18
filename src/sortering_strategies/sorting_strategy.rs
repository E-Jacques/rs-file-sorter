use std::fs::File;
use std::sync::{Arc, Mutex};
pub struct SortingStrategy {
    pub method_chain: Vec<fn(&File) -> String>,
}

impl SortingStrategy {
    pub fn set_method_chain (&mut self, method_chain: Vec<fn(&File) -> String>) {
        self.method_chain = method_chain;
    }

    pub fn new () -> Self {
        Self { method_chain: vec![] }
    }

    pub fn iter (&self, file: File) -> impl Iterator<Item=String> + '_ {
        let file_mutex = Arc::new(Mutex::new(file));

        self.method_chain.iter().map(move |func| {
            let file_clone = Arc::clone(&file_mutex);
            let file_lock = file_clone.lock().unwrap();
            func(&*file_lock)
        })
    }
}