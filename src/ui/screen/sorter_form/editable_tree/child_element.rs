use iced::Element;

use crate::{
    core::{
        parameter::{StrategyParameter, StrategyParameterKind},
        validation::ParameterDetail,
    },
    sorting_strategies::strategy_catalog::StrategyCatalog,
    ui::{
        screen::sorter_form::editable_tree::{
            editable_tree::EditableTree,
            editable_tree_item_combo_box::EditableTreeItemComboBox,
            editable_tree_item_number::EditableTreeItemNumber,
            editable_tree_item_text_input::EditableTreeItemTextInput,
            shared::{ParameterInput, TreeInputMessage, TreeItemMessage},
        },
        template::strategy_payload::ParameterValue,
    },
};

#[derive(Debug, Clone)]
pub enum ChildElement {
    StrategyParameter(EditableTree),
    StringParameter(Box<dyn ParameterInput<String>>),
    NumberParameter(Box<dyn ParameterInput<usize>>),
}

impl ChildElement {
    pub fn view<'a>(&'a self, name: &'a String) -> Element<'a, TreeItemMessage> {
        match self {
            ChildElement::StrategyParameter(element) => {
                element.view().map(map_message(name.to_string()))
            }
            ChildElement::StringParameter(element) => {
                element.view().map(map_message(name.to_string()))
            }
            ChildElement::NumberParameter(element) => {
                element.view().map(map_message(name.to_string()))
            }
        }
    }

    pub fn update(&mut self, message: TreeInputMessage) {
        match message {
            TreeInputMessage::EditableTree(tree_message) => {
                if let ChildElement::StrategyParameter(element) = self {
                    element.update(tree_message);
                }
            }
            TreeInputMessage::TextInput(tree_text_input_message) => match self {
                ChildElement::StringParameter(element) => {
                    element.update(tree_text_input_message);
                }
                ChildElement::NumberParameter(element) => {
                    element.update(tree_text_input_message);
                }
                _ => (),
            },
        }
    }

    pub fn create(validator: ParameterDetail, strategy_catalog: StrategyCatalog) -> Self {
        match validator.kind {
            StrategyParameterKind::Strategy => {
                ChildElement::StrategyParameter(EditableTree::new(strategy_catalog))
            }
            StrategyParameterKind::SingleString => ChildElement::StringParameter(Box::new(
                EditableTreeItemTextInput::new("Insert a value here".to_string()),
            )),
            StrategyParameterKind::Choice(items) => {
                let default_value: Option<String> = match validator.default_value {
                    Some(StrategyParameter::SingleString(value)) => Some(value),
                    _ => None,
                };
                ChildElement::StringParameter(Box::new(EditableTreeItemComboBox::new(
                    "Select an option".to_string(),
                    default_value,
                    items,
                )))
            }
            StrategyParameterKind::Number => {
                let default_value: Option<usize> = match validator.default_value {
                    Some(StrategyParameter::Number(value)) => Some(value),
                    _ => None,
                };
                ChildElement::NumberParameter(Box::new(EditableTreeItemNumber::new(
                    "Insert a number here".to_string(),
                    default_value,
                )))
            }
        }
    }

    pub fn strategy_parameter(&self) -> Option<StrategyParameter> {
        match self {
            ChildElement::StrategyParameter(screen) => {
                Some(StrategyParameter::Strategy(screen.get_sorting_strategies()))
            }
            ChildElement::StringParameter(screen) => {
                screen.get_value().map(StrategyParameter::SingleString)
            }
            ChildElement::NumberParameter(screen) => {
                screen.get_value().map(StrategyParameter::Number)
            }
        }
    }
}

fn map_message<T: Into<TreeInputMessage>>(name: String) -> impl Fn(T) -> TreeItemMessage {
    move |child_message| {
        TreeItemMessage::ParameterChanged(name.clone(), Box::new(child_message.into()))
    }
}

impl From<ParameterValue> for ChildElement {
    fn from(value: ParameterValue) -> Self {
        match value {
            ParameterValue::String(str) => {
                // TODO : refactor to have defautl value
                let mut el = EditableTreeItemTextInput::new("Select an option".to_string());
                el.update(super::shared::TreeTextInputMessage::ValueUpdate(
                    str.clone(),
                ));
                ChildElement::StringParameter(Box::new(el))
            }
            ParameterValue::Integer(num) => ChildElement::NumberParameter(Box::new(
                EditableTreeItemNumber::new("Insert a number here".to_string(), Some(num)),
            )),
            ParameterValue::Array(arr) => ChildElement::StrategyParameter(EditableTree::from(arr)),
            _ => {
                panic!("Unsupported ParameterValue type for ChildElement conversion");
            }
        }
    }
}
