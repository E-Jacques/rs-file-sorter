use iced::widget::combo_box;

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
