use iced::{widget, Element};

use crate::ui::{
    custom_theme,
    screen::sorter_form::editable_tree::shared::{ParameterInput, TreeTextInputMessage},
};

#[derive(Debug, Clone)]
pub struct EditableTreeItemNumber {
    value: Option<usize>,
    placeholder: String,
}

impl EditableTreeItemNumber {
    pub fn new(placeholder: String, default_value: Option<usize>) -> EditableTreeItemNumber {
        EditableTreeItemNumber {
            placeholder,
            value: default_value,
        }
    }
}

impl ParameterInput<usize> for EditableTreeItemNumber {
    fn view(&self) -> Element<'_, TreeTextInputMessage> {
        let display_value = self.value.map(|v| v.to_string()).unwrap_or("".to_string());
        widget::text_input(&self.placeholder, &display_value)
            .on_input(TreeTextInputMessage::ValueUpdate)
            .style(custom_theme::TextInput::style)
            .into()
    }

    fn update(&mut self, message: TreeTextInputMessage) {
        match message {
            TreeTextInputMessage::ValueUpdate(value) => self.value = value.parse().ok(),
        }
    }

    fn get_value(&self) -> Option<usize> {
        self.value
    }

    fn clone_box(&self) -> Box<dyn ParameterInput<usize>> {
        Box::new(self.clone())
    }
}
