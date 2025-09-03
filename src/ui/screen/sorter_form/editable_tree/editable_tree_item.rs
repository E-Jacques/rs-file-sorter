use std::collections::HashMap;

use iced::{
    widget::{column, combo_box, container, row, text, Column, ComboBox},
    Alignment, Element, Length,
};

use crate::{
    core::{
        strategy::Strategy,
        parameter::{StrategyParameter, StrategyParameterKind},
    },
    sorting_strategies::strategy_catalog::StrategyCatalog,
    ui::{
        custom_theme,
        widget::{button::icon_button::icon_button, icon},
    },
};

use super::{
    editable_tree::EditableTree,
    editable_tree_item_combo_box::EditableTreeItemComboBox,
    editable_tree_item_text_input::EditableTreeItemTextInput,
    shared::{
        DirectoryMovement, StrategyOptions, StringParameterInput, TreeInputMessage, TreeItemMessage,
    },
};

#[derive(Debug, Clone)]
enum ChildElement {
    StrategyParameter(EditableTree),
    StringParameter(Box<dyn StringParameterInput>),
}

#[derive(Debug, Clone)]
pub struct EditableTreeItem {
    selected_strategy: Option<String>,
    strategy_options: StrategyOptions,
    strategy_catalog: StrategyCatalog,
    child_elements: HashMap<String, ChildElement>,
}

impl EditableTreeItem {
    pub fn new(strategy_catalog: StrategyCatalog) -> Self {
        let strategy_options = combo_box::State::new(strategy_catalog.get_names());
        EditableTreeItem {
            selected_strategy: None,
            strategy_catalog,
            strategy_options,
            child_elements: HashMap::new(),
        }
    }

    pub fn view(&self) -> Element<'_, TreeItemMessage> {
        let delete_btn: Element<'_, TreeItemMessage> = icon_button(icon::DELETE)
            .on_press(TreeItemMessage::DirectoryRemoved)
            .into();
        let up_btn: Element<'_, TreeItemMessage> = icon_button(icon::ARROW_UP)
            .on_press(TreeItemMessage::MoveDirectory(DirectoryMovement::Up))
            .into();
        let down_btn: Element<'_, TreeItemMessage> = icon_button(icon::ARROW_DOWN)
            .on_press(TreeItemMessage::MoveDirectory(DirectoryMovement::Down))
            .into();

        let strategy_name_input: Element<'_, TreeItemMessage> = ComboBox::new(
            &self.strategy_options,
            "Select a strategy",
            self.selected_strategy.as_ref(),
            move |selected_strategy| TreeItemMessage::StrategyChanged(selected_strategy),
        )
        .input_style(custom_theme::TextInput::style)
        .into();

        let header: Element<'_, TreeItemMessage> = row![
            strategy_name_input,
            row![delete_btn, up_btn, down_btn].spacing(4)
        ]
        .align_y(Alignment::Center)
        .spacing(24)
        .width(Length::Fill)
        .into();
        let body = self.render_all_elements();

        container(column![header, body].spacing(8))
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(16)
            .style(|_| {
                let mut style = container::Style::default();
                style.border = custom_theme::border_style();

                style
            })
            .into()
    }

    fn render_all_elements(&self) -> Element<'_, TreeItemMessage> {
        let mut target_column = Column::new();

        for (name, screen) in &self.child_elements {
            let param_name_text: Element<'_, TreeItemMessage> = text(name).into();
            let editable_tree_element: Element<'_, TreeItemMessage> = match screen {
                ChildElement::StrategyParameter(editable_tree_item) => {
                    editable_tree_item.view().map(move |child_message| {
                        TreeItemMessage::ParameterChanged(
                            name.clone(),
                            Box::new(TreeInputMessage::EditableTree(child_message)),
                        )
                    })
                }
                ChildElement::StringParameter(element) => {
                    element.view().map(move |child_message| {
                        TreeItemMessage::ParameterChanged(
                            name.clone(),
                            Box::new(TreeInputMessage::TextInput(child_message)),
                        )
                    })
                }
            };
            let col: Element<'_, TreeItemMessage> = Column::new()
                .push(param_name_text)
                .push(editable_tree_element)
                .spacing(4)
                .into();
            target_column = target_column.push(col);
        }

        container(target_column).width(Length::Fill).into()
    }

    pub fn update(&mut self, message: TreeItemMessage) {
        match message {
            TreeItemMessage::StrategyChanged(strategy) => {
                self.selected_strategy = Some(strategy);
                self.set_strategy_properties_setter();
            }
            TreeItemMessage::ParameterChanged(parameter_name, parameter_message) => {
                match *parameter_message {
                    TreeInputMessage::EditableTree(tree_message) => {
                        if let Some(ChildElement::StrategyParameter(element)) =
                            self.child_elements.get_mut(&parameter_name)
                        {
                            element.update(tree_message);
                        }
                    }
                    TreeInputMessage::TextInput(tree_text_input_message) => {
                        if let Some(ChildElement::StringParameter(element)) =
                            self.child_elements.get_mut(&parameter_name)
                        {
                            element.update(tree_text_input_message);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn set_strategy_properties_setter(&mut self) {
        self.child_elements.clear();

        if let Some(strategy) = self.get_sorting_strategy() {
            for validator in strategy.parameter_details() {
                match validator.kind {
                    StrategyParameterKind::Strategy => {
                        self.child_elements.insert(
                            validator.name.clone(),
                            ChildElement::StrategyParameter(EditableTree::new(
                                self.strategy_catalog.clone(),
                            )),
                        );
                    }
                    StrategyParameterKind::SingleString => {
                        self.child_elements.insert(
                            validator.name.clone(),
                            ChildElement::StringParameter(Box::new(
                                EditableTreeItemTextInput::new("Insert a value here".to_string()),
                            )),
                        );
                    }
                    StrategyParameterKind::Choice(items) => {
                        let default_value: Option<String> = match validator.default_value {
                            Some(StrategyParameter::SingleString(value)) => Some(value),
                            _ => None,
                        };
                        self.child_elements.insert(
                            validator.name.clone(),
                            ChildElement::StringParameter(Box::new(EditableTreeItemComboBox::new(
                                "Select an option".to_string(),
                                default_value,
                                items,
                            ))),
                        );
                    }
                }
            }
        }
    }

    pub fn get_sorting_strategy(&self) -> Option<Box<dyn Strategy>> {
        let name = self.selected_strategy.as_ref()?;
        let mut strategy = self.strategy_catalog.get_strategy(name)?;

        for (key, child_element) in &self.child_elements {
            let maybe_value: Option<StrategyParameter> = match child_element {
                ChildElement::StrategyParameter(screen) => {
                    Some(StrategyParameter::Strategy(screen.get_sorting_strategies()))
                }
                ChildElement::StringParameter(screen) => {
                    screen.get_value().map(StrategyParameter::SingleString)
                }
            };

            if let Some(value) = maybe_value {
                strategy.add_parameter(key.clone(), value);
            }
        }

        Some(strategy)
    }
}
