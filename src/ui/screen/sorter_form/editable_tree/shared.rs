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

#[derive(Debug, Clone)]
pub enum TreeItemMessage {
    DirectoryRemoved,
    StrategyChanged(String),
    MoveDirectory(DirectoryMovement),
    ParameterChanged(String, Box<TreeInputMessage>),
}

pub trait StringParameterInput: std::fmt::Debug {
    fn view(&self) -> Element<'_, TreeTextInputMessage>;
    fn update(&mut self, msg: TreeTextInputMessage);
    fn get_value(&self) -> Option<String>;
    fn clone_box(&self) -> Box<dyn StringParameterInput>;
}

impl Clone for Box<dyn StringParameterInput> {
    fn clone(&self) -> Box<dyn StringParameterInput> {
        self.clone_box()
    }
}

pub trait NumberParameterInput: std::fmt::Debug {
    fn view(&self) -> Element<'_, TreeTextInputMessage>;
    fn update(&mut self, msg: TreeTextInputMessage);
    fn get_value(&self) -> Option<usize>;
    fn clone_box(&self) -> Box<dyn NumberParameterInput>;
}

impl Clone for Box<dyn NumberParameterInput> {
    fn clone(&self) -> Box<dyn NumberParameterInput> {
        self.clone_box()
    }
}
