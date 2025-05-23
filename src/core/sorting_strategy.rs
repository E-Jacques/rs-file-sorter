use std::fs::File;
use std::sync::{Arc, Mutex};

type SortingStrategyAction = Box<dyn Fn(&File) -> String>;

#[derive(Clone)]
pub struct SortingStrategy {
    pub action: Arc<SortingStrategyAction>,
    pub name: String,
}

impl std::fmt::Debug for SortingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortingStrategy")
            .field("name", &self.name)
            .finish()
    }
}

impl SortingStrategy {
    pub fn new(name: &str, action: SortingStrategyAction) -> SortingStrategy {
        SortingStrategy {
            action: Arc::new(action),
            name: name.to_string(),
        }
    }

    pub(crate) fn apply(&self, file: &File) -> String {
        println!("{:#?}", file);
        let file_mutex = Arc::new(Mutex::new(file));
        let file_clone = Arc::clone(&file_mutex);
        let file_lock = file_clone.lock().unwrap();
        let result = (self.action)(&*file_lock);
        println!("{:#?}", result);

        return result;
    }
}
