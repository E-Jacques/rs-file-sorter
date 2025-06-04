use iced::{
    widget::{button, combo_box, row, ComboBox},
    Element, Length,
};

use crate::{
    core::sorting_strategy::SortingStrategy, sorting_strategies::strategy_catalog::StrategyCatalog,
    ui::widgets::icon,
};

use super::shared::{DirectoryMovement, ItemMessage, StrategyOptions, TreeItem};

#[derive(Debug, Clone)]
pub struct DefaultEditableTreeItem {
    selected_strategy: Option<String>,
    strategy_options: StrategyOptions,
    strategy_catalog: StrategyCatalog,
}

impl DefaultEditableTreeItem {
    pub fn new(strategy_catalog: StrategyCatalog) -> Self {
        let strategy_options = combo_box::State::new(strategy_catalog.get_names());
        DefaultEditableTreeItem {
            selected_strategy: None,
            strategy_catalog,
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
            ItemMessage::StrategyChanged(strategy) => {
                self.selected_strategy = Some(strategy);
            }
            _ => (),
        }
    }

    fn get_sorting_strategy(&self) -> Option<SortingStrategy> {
        self.selected_strategy
            .as_ref()
            .and_then(|name| self.strategy_catalog.get_strategy(name))
    }

    fn box_clone(&self) -> Box<dyn TreeItem<ItemMessage>> {
        Box::new(Self {
            selected_strategy: self.selected_strategy.clone(),
            strategy_catalog: self.strategy_catalog.clone(),
            strategy_options: self.strategy_options.clone(),
        })
    }
}
