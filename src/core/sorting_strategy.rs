use std::fs::File;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SortingStrategy<'a> {
    pub action: fn(&File) -> String,
    pub name: &'a str,
}

impl<'a> SortingStrategy<'a> {
    pub fn new(name: &str, action: fn(&File) -> String) -> SortingStrategy {
        SortingStrategy { action, name }
    }

    pub fn apply(&self, file: &File) -> String {
        println!("{:#?}", file);
        let file_mutex = Arc::new(Mutex::new(file));
        let file_clone = Arc::clone(&file_mutex);
        let file_lock = file_clone.lock().unwrap();
        let result = (self.action)(&*file_lock);
        println!("{:#?}", result);

        return result;
    }
}
