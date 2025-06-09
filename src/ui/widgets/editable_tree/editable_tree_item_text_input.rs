use iced::{widget::text_input, Element};

use crate::ui::widgets::editable_tree::shared::TreeTextInputMessage;

#[derive(Debug, Clone)]
pub struct EditableTreeItemTextInput {
    value: String,
    placeholder: String,
}

impl EditableTreeItemTextInput {
    pub fn new(placeholder: String) -> EditableTreeItemTextInput {
        EditableTreeItemTextInput {
            placeholder,
            value: String::default(),
        }
    }

    pub fn view(&self) -> Element<'_, TreeTextInputMessage> {
        text_input(&self.placeholder, &self.value)
            .on_input(TreeTextInputMessage::ValueUpdate)
            .into()
    }

    pub fn update(&mut self, message: TreeTextInputMessage) {
        match message {
            TreeTextInputMessage::ValueUpdate(value) => self.value = value,
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}
