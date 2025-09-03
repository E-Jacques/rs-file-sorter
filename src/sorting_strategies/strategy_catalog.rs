use crate::core::strategy::Strategy;

#[derive(Default, Clone, Debug)]
pub struct StrategyCatalog {
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategyCatalog {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        StrategyCatalog { strategies }
    }

    pub fn get_strategy(&self, strategy_name: &String) -> Option<Box<dyn Strategy>> {
        self.strategies
            .iter()
            .find(|strategy| strategy.name() == *strategy_name)
            .cloned()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.strategies
            .iter()
            .map(|strategy| strategy.name())
            .collect()
    }

    pub fn with(&self, catalog: &StrategyCatalog) -> StrategyCatalog {
        StrategyCatalog::new([self.strategies.clone(), catalog.strategies.clone()].concat())
    }
}

impl From<Vec<Box<dyn Strategy>>> for StrategyCatalog {
    fn from(strategies: Vec<Box<dyn Strategy>>) -> Self {
        StrategyCatalog::new(strategies)
    }
}
