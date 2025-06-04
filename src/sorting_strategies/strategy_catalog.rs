use crate::core::sorting_strategy::SortingStrategy;

#[derive(Default, Clone, Debug)]
pub struct StrategyCatalog {
    strategies: Vec<SortingStrategy>,
}

impl StrategyCatalog {
    pub fn new(strategies: Vec<SortingStrategy>) -> Self {
        StrategyCatalog { strategies }
    }

    pub fn get_strategy(&self, strategy_name: &String) -> Option<SortingStrategy> {
        self.strategies
            .iter()
            .find(|strategy| strategy.name == *strategy_name)
            .cloned()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.strategies
            .iter()
            .map(|strategy| strategy.name.clone())
            .collect()
    }

    pub fn with(&self, catalog: &StrategyCatalog) -> StrategyCatalog {
        StrategyCatalog::new([self.strategies.clone(), catalog.strategies.clone()].concat())
    }
}

impl From<Vec<SortingStrategy>> for StrategyCatalog {
    fn from(strategies: Vec<SortingStrategy>) -> Self {
        StrategyCatalog::new(strategies)
    }
}
