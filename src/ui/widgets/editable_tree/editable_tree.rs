use iced::{
    border::Radius,
    widget::{button, column, combo_box, container},
    Border, Color, Element, Length,
};

use crate::{
    core::sorting_strategy::SortingStrategy,
    sorting_strategies::{get_month_sorting_strategy, get_year_sorting_strategy},
    utils::string_manipulator::random_string,
};

use super::{
    editable_tree_item::{DirectoryMovement, EditableTreeItem, Message as ItemMessage},
    shared::StrategyOptions,
};

#[derive(Debug, Clone)]
pub struct EditableTree {
    items: Vec<Directory>,
    strategies_options: StrategyOptions,
    strategies_list: Vec<SortingStrategy>,
}
#[derive(Debug, Clone)]
struct Directory {
    id: String,
    element: EditableTreeItem,
    strategy: Option<SortingStrategy>,
}

#[derive(Debug, Clone)]
pub enum Message {
    DirectoryAdded,
    ItemEvent(String, ItemMessage),
}

impl Default for EditableTree {
    fn default() -> Self {
        EditableTree::new()
    }
}

impl EditableTree {
    fn new() -> Self {
        let strategy_list: Vec<SortingStrategy> =
            vec![get_month_sorting_strategy(), get_year_sorting_strategy()];
        EditableTree {
            items: vec![],
            strategies_options: combo_box::State::new(
                strategy_list
                    .iter()
                    .map(|s| s.name.clone())
                    .collect::<Vec<String>>(),
            ),
            strategies_list: strategy_list,
        }
    }

    fn add_directory(&mut self) {
        let new_directory = Directory {
            id: random_string(10),
            element: EditableTreeItem::new(self.strategies_options.clone()),
            strategy: None,
        };
        self.items.push(new_directory);
    }

    fn remove_directory(&mut self, id: String) {
        // Remove directory logic
        if let Some(index) = self.items.iter().position(|dir| dir.id == id) {
            self.items.remove(index);
        } else {
            println!("Directory with id {} not found", id);
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let columns = self
            .items
            .iter()
            .map(|dir: &Directory| -> Element<'_, Message> {
                dir.element
                    .view()
                    .map(move |item_message| Message::ItemEvent(dir.id.clone(), item_message))
            });

        let add_directory_btn: Element<'_, Message> = button("Add Directory")
            .on_press(Message::DirectoryAdded)
            .width(Length::Fill)
            .into();
        let content = column(columns.into_iter())
            .push(add_directory_btn)
            .padding(20)
            .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(|_| {
                container::Style::default().border(Border {
                    color: Color::BLACK,
                    width: 1.0,
                    radius: Radius::new(4),
                })
            })
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DirectoryAdded => self.add_directory(),
            Message::ItemEvent(id, item_message) => {
                // update the related item element
                self.items
                    .iter_mut()
                    .find(|item| item.id == id)
                    .unwrap()
                    .element
                    .update(item_message.clone());

                // react on the parent side
                match item_message {
                    ItemMessage::DirectoryRemoved => self.remove_directory(id),
                    ItemMessage::MoveDirectory(movement) => {
                        self.move_directory(id, movement);
                    }
                    ItemMessage::StrategyChanged(name) => {
                        self.change_directory_name(id, name.clone());
                    }
                }
            }
        }
    }

    fn change_directory_name(&mut self, id: String, name: String) {
        if let Some(dir) = self.items.iter_mut().find(|dir| dir.id == id) {
            if let Some(strategy) = self.strategies_list.iter().find(|s| s.name == name) {
                dir.strategy = Some(strategy.clone());
            } else {
                println!("Strategy already set for directory {}", dir.id);
            }
        } else {
            println!("Directory with id {} not found", id);
        }
    }

    fn move_directory(&mut self, id: String, movement: DirectoryMovement) {
        let index = self.items.iter().position(|dir| dir.id == id);
        if let Some(index) = index {
            match movement {
                DirectoryMovement::Up => {
                    if index > 0 {
                        self.items.swap(index, index - 1);
                    }
                }
                DirectoryMovement::Down => {
                    if index < self.items.len() - 1 {
                        self.items.swap(index, index + 1);
                    }
                }
            }
        }
    }

    pub fn get_sorting_strategies(&self) -> Vec<SortingStrategy> {
        self.items
            .iter()
            .filter_map(|dir| dir.strategy.clone())
            .collect()
    }
}
