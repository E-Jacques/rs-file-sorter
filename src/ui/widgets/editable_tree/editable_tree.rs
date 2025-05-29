use std::fmt::Debug;

use iced::{
    border::Radius,
    widget::{button, column, container},
    Border, Color, Element, Length,
};

use crate::{
    core::sorting_strategy::SortingStrategy,
    sorting_strategies::{get_month_sorting_strategy, get_year_sorting_strategy},
    utils::string_manipulator::random_string,
};

use super::{
    default_editable_tree_item::{
        DefaultEditableTreeItem, DirectoryMovement, Message as ItemMessage,
    },
    shared::TreeItem,
};

#[derive(Debug, Clone)]
pub struct EditableTree {
    items: Vec<Directory>,
    strategies_list: Vec<SortingStrategy>,
}
#[derive(Debug)]
struct Directory {
    id: String,
    element: Box<dyn TreeItem<ItemMessage>>,
}

// Manual Clone implementation for Directory
impl Clone for Directory {
    fn clone(&self) -> Self {
        Directory {
            id: self.id.clone(),
            element: self.element.box_clone(),
        }
    }
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
        let strategies_list: Vec<SortingStrategy> =
            vec![get_month_sorting_strategy(), get_year_sorting_strategy()];
        EditableTree {
            items: vec![],
            strategies_list,
        }
    }

    fn add_directory(&mut self) {
        let new_directory = Directory {
            id: random_string(10),
            element: Box::new(DefaultEditableTreeItem::new(self.strategies_list.clone())),
        };
        self.items.push(new_directory);
    }

    fn remove_item(&mut self, id: String) {
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
                    ItemMessage::DirectoryRemoved => self.remove_item(id),
                    ItemMessage::MoveDirectory(movement) => {
                        self.move_item(id, movement);
                    }
                    ItemMessage::StrategyChanged(_) => (),
                }
            }
        }
    }

    fn move_item(&mut self, id: String, movement: DirectoryMovement) {
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
            .filter_map(|dir| dir.element.get_sorting_strategy())
            .collect()
    }
}
