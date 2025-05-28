use iced::{
    widget::{button, combo_box, row},
    Element, Length,
};

use crate::ui::widgets::icon;

use super::shared::StrategyOptions;

#[derive(Debug, Clone)]
pub struct EditableTreeItem {
    selected_strategy: Option<String>,
    strategy_options: StrategyOptions,
}

#[derive(Debug, Clone)]
pub enum DirectoryMovement {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum Message {
    DirectoryRemoved,
    StrategyChanged(String),
    MoveDirectory(DirectoryMovement),
}

impl EditableTreeItem {
    pub fn new(strategy_options: StrategyOptions) -> Self {
        EditableTreeItem {
            selected_strategy: None,
            strategy_options: strategy_options,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let delete_btn: Element<'_, Message> = button(icon::icon(icon::DELETE))
            .on_press(Message::DirectoryRemoved)
            .into();
        let up_btn: Element<'_, Message> = button(icon::icon(icon::ARROW_UP))
            .on_press(Message::MoveDirectory(DirectoryMovement::Up))
            .into();
        let down_btn: Element<'_, Message> = button(icon::icon(icon::ARROW_DOWN))
            .on_press(Message::MoveDirectory(DirectoryMovement::Down))
            .into();

        let strategy_name_input = combo_box(
            &self.strategy_options,
            "Select a strategy",
            self.selected_strategy.as_ref(),
            move |selected_strategy| Message::StrategyChanged(selected_strategy),
        );

        row![strategy_name_input, delete_btn, up_btn, down_btn]
            .spacing(10)
            .width(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DirectoryRemoved => (),
            Message::StrategyChanged(strategy) => {
                self.selected_strategy = Some(strategy);
            }
            Message::MoveDirectory(_) => (),
        }
    }
}
