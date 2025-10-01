use std::collections::HashMap;

use iced::{
    widget::{column, combo_box, container, row, text, Column, ComboBox},
    Alignment, Element, Length,
};

use crate::{
    core::strategy::Strategy,
    sorting_strategies::strategy_catalog::StrategyCatalog,
    ui::{
        custom_theme,
        screen::sorter_form::editable_tree::child_element::ChildElement,
        widget::{button::icon_button::icon_button, icon},
    },
};

use super::shared::{DirectoryMovement, StrategyOptions, TreeItemMessage};

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

        for (name, child_element) in &self.child_elements {
            let param_name_text: Element<'_, TreeItemMessage> = text(name).into();
            let editable_tree_element: Element<'_, TreeItemMessage> = child_element.view(name);
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
                if let Some(child_element) = self.child_elements.get_mut(&parameter_name) {
                    child_element.update(*parameter_message);
                }
            }
            _ => (),
        }
    }

    fn set_strategy_properties_setter(&mut self) {
        self.child_elements.clear();

        if let Some(strategy) = self.get_sorting_strategy() {
            for validator in strategy.parameter_details() {
                self.child_elements.insert(
                    validator.name.clone(),
                    ChildElement::create(validator, self.strategy_catalog.clone()),
                );
            }
        }
    }

    pub fn get_sorting_strategy(&self) -> Option<Box<dyn Strategy>> {
        let name = self.selected_strategy.as_ref()?;
        let mut strategy = self.strategy_catalog.get_strategy(name)?;

        for (key, child_element) in &self.child_elements {
            if let Some(value) = child_element.strategy_parameter() {
                strategy.add_parameter(key.clone(), value);
            }
        }

        Some(strategy)
    }
}
