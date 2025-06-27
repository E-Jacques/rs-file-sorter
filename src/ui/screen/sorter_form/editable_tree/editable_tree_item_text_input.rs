use iced::{widget, Element};

use crate::ui::custom_theme;

use super::shared::{StringParameterInput, TreeTextInputMessage};

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
}

impl StringParameterInput for EditableTreeItemTextInput {
    fn view(&self) -> Element<'_, TreeTextInputMessage> {
        widget::text_input(&self.placeholder, &self.value)
            .on_input(TreeTextInputMessage::ValueUpdate)
            .style(custom_theme::TextInput::style)
            .into()
    }

    fn update(&mut self, message: TreeTextInputMessage) {
        match message {
            TreeTextInputMessage::ValueUpdate(value) => self.value = value,
        }
    }

    fn get_value(&self) -> Option<String> {
        Some(self.value.clone())
    }

    fn clone_box(&self) -> Box<dyn StringParameterInput> {
        Box::new(self.clone())
    }
}
