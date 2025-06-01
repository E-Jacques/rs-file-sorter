use iced::{
    widget::{button, combo_box, row, ComboBox},
    Element, Length,
};

use crate::{core::sorting_strategy::SortingStrategy, ui::widgets::icon};

use super::shared::{DirectoryMovement, ItemMessage, StrategyOptions, TreeItem};

#[derive(Debug, Clone)]
pub struct DefaultEditableTreeItem {
    selected_strategy: Option<String>,
    strategy_options: StrategyOptions,
    strategy_list: Vec<SortingStrategy>,
}

impl DefaultEditableTreeItem {
    pub fn new(strategy_list: Vec<SortingStrategy>) -> Self {
        let strategy_options = combo_box::State::new(
            strategy_list
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<String>>(),
        );
        DefaultEditableTreeItem {
            selected_strategy: None,
            strategy_list,
            strategy_options,
        }
    }
}

impl TreeItem<ItemMessage> for DefaultEditableTreeItem {
    fn view(&self) -> Element<'_, ItemMessage> {
        let delete_btn: Element<'_, ItemMessage> = button(icon::icon(icon::DELETE))
            .on_press(ItemMessage::DirectoryRemoved)
            .into();
        let up_btn: Element<'_, ItemMessage> = button(icon::icon(icon::ARROW_UP))
            .on_press(ItemMessage::MoveDirectory(DirectoryMovement::Up))
            .into();
        let down_btn: Element<'_, ItemMessage> = button(icon::icon(icon::ARROW_DOWN))
            .on_press(ItemMessage::MoveDirectory(DirectoryMovement::Down))
            .into();

        let strategy_name_input: Element<'_, ItemMessage> = ComboBox::new(
            &self.strategy_options,
            "Select a strategy",
            self.selected_strategy.as_ref(),
            move |selected_strategy| ItemMessage::StrategyChanged(selected_strategy),
        )
        .into();

        row![strategy_name_input, delete_btn, up_btn, down_btn]
            .spacing(10)
            .width(Length::Fill)
            .into()
    }

    fn update(&mut self, message: ItemMessage) {
        match message {
            ItemMessage::DirectoryRemoved => (),
            ItemMessage::StrategyChanged(strategy) => {
                self.selected_strategy = Some(strategy);
            }
            ItemMessage::MoveDirectory(_) => (),
            ItemMessage::NestedEditableTreeMessage(_) => (),
        }
    }

    fn get_sorting_strategy(&self) -> Option<SortingStrategy> {
        match self.selected_strategy.clone() {
            Some(selected_strategy) => self
                .strategy_list
                .iter()
                .find(|strategy| strategy.name == selected_strategy)
                .cloned(),
            None => None,
        }
    }

    fn box_clone(&self) -> Box<dyn TreeItem<ItemMessage>> {
        Box::new(Self {
            selected_strategy: self.selected_strategy.clone(),
            strategy_list: self.strategy_list.clone(),
            strategy_options: self.strategy_options.clone(),
        })
    }
}
