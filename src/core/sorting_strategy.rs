use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};

type SortingStrategyAction = fn(&File, &HashMap<String, StrategyParameter>) -> String;

#[derive(Clone)]
pub enum StrategyParameter {
    Strategy(Vec<Box<SortingStrategy>>),
}

#[derive(Clone)]
pub struct SortingStrategy {
    pub action: SortingStrategyAction,
    pub name: String,
    pub parameters: HashMap<String, StrategyParameter>,
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
            action: action,
            name: name.to_string(),
            parameters: HashMap::new(),
        }
    }

    pub fn add_parameter(&mut self, key: String, value: StrategyParameter) {
        self.parameters.insert(key, value);
    }

    pub(crate) fn apply(&self, file: &File) -> String {
        println!("{:#?}", file);
        let file_mutex = Arc::new(Mutex::new(file));
        let file_clone = Arc::clone(&file_mutex);
        let file_lock = file_clone.lock().unwrap();
        let result = (self.action)(&*file_lock, &self.parameters.clone());
        println!("{:#?}", result);

        return result;
    }
}
