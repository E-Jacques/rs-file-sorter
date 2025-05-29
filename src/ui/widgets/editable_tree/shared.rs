use iced::{widget::combo_box, Element};

use crate::core::sorting_strategy::SortingStrategy;

pub type StrategyOptions = combo_box::State<String>;

pub trait TreeItem<M>: std::fmt::Debug {
    fn view(&self) -> Element<'_, M>;
    fn update(&mut self, msg: M);
    fn box_clone(&self) -> Box<dyn TreeItem<M>>;
    fn get_sorting_strategy(&self) -> Option<SortingStrategy>;
}
