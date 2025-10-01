use iced::{widget::combo_box, Element};

pub type StrategyOptions = combo_box::State<String>;

#[derive(Debug, Clone)]
pub enum DirectoryMovement {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum TreeMessage {
    AddEmptyItem,
    ItemEvent(String, TreeItemMessage),
}
#[derive(Debug, Clone)]
pub enum TreeTextInputMessage {
    ValueUpdate(String),
}

#[derive(Debug, Clone)]
pub enum TreeInputMessage {
    EditableTree(TreeMessage),
    TextInput(TreeTextInputMessage),
}

impl Into<TreeInputMessage> for TreeMessage {
    fn into(self) -> TreeInputMessage {
        TreeInputMessage::EditableTree(self)
    }
}

impl Into<TreeInputMessage> for TreeTextInputMessage {
    fn into(self) -> TreeInputMessage {
        TreeInputMessage::TextInput(self)
    }
}

#[derive(Debug, Clone)]
pub enum TreeItemMessage {
    DirectoryRemoved,
    StrategyChanged(String),
    MoveDirectory(DirectoryMovement),
    ParameterChanged(String, Box<TreeInputMessage>),
}

pub trait ParameterInput<T>: std::fmt::Debug {
    fn view(&self) -> Element<'_, TreeTextInputMessage>;
    fn update(&mut self, msg: TreeTextInputMessage);
    fn get_value(&self) -> Option<T>;
    fn clone_box(&self) -> Box<dyn ParameterInput<T>>;
}

impl<T> Clone for Box<dyn ParameterInput<T>> {
    fn clone(&self) -> Box<dyn ParameterInput<T>> {
        self.clone_box()
    }
}
