use iced::{widget, Element};

use crate::ui::{custom_theme, screen::sorter_form::editable_tree::shared::ParameterInput};

use super::shared::TreeTextInputMessage;

#[derive(Debug, Clone)]
pub struct EditableTreeItemComboBox {
    state: widget::combo_box::State<String>,
    value: Option<String>,
    placeholder: String,
}

impl EditableTreeItemComboBox {
    pub fn new(
        placeholder: String,
        default_value: Option<String>,
        choices: Vec<String>,
    ) -> EditableTreeItemComboBox {
        EditableTreeItemComboBox {
            placeholder,
            value: default_value,
            state: widget::combo_box::State::new(choices),
        }
    }
}

impl ParameterInput<String> for EditableTreeItemComboBox {
    fn view(&self) -> Element<'_, TreeTextInputMessage> {
        widget::combo_box(
            &self.state,
            &self.placeholder,
            self.value.as_ref(),
            TreeTextInputMessage::ValueUpdate,
        )
        .input_style(custom_theme::TextInput::style)
        .into()
    }

    fn update(&mut self, message: TreeTextInputMessage) {
        match message {
            TreeTextInputMessage::ValueUpdate(value) => self.value = Some(value),
        }
    }

    fn get_value(&self) -> Option<String> {
        self.value.clone()
    }
    fn clone_box(&self) -> Box<dyn ParameterInput<String>> {
        Box::new(self.clone())
    }
}
