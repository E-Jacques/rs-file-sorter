use iced::{widget::combo_box, Element};

use crate::core::sorting_strategy::SortingStrategy;

pub type StrategyOptions = combo_box::State<String>;

pub trait TreeItem<M>: std::fmt::Debug {
    fn view(&self) -> Element<'_, M>;
    fn update(&mut self, msg: M);
    fn box_clone(&self) -> Box<dyn TreeItem<M>>;
    fn get_sorting_strategy(&self) -> Option<SortingStrategy>;
}

#[derive(Debug, Clone)]
pub enum DirectoryMovement {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum Message {
    DirectoryAdded,
    ItemEvent(String, ItemMessage),
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    DirectoryRemoved,
    StrategyChanged(String),
    MoveDirectory(DirectoryMovement),
    NestedEditableTreeMessage(Box<Message>),
}
