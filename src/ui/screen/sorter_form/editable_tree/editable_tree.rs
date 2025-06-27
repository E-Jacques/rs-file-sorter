use std::fmt::Debug;

use iced::{widget::column, Element, Length};

use crate::{
    core::sorting_strategy::SortingStrategy, sorting_strategies::strategy_catalog::StrategyCatalog,
    ui::widget::button::primary_button::primary_button, utils::string_manipulator::random_string,
};

use super::{
    editable_tree_item::EditableTreeItem,
    shared::{DirectoryMovement, TreeItemMessage, TreeMessage},
};

#[derive(Debug, Clone)]
pub struct EditableTree {
    items: Vec<Directory>,
    strategy_catalog: StrategyCatalog,
}

#[derive(Debug, Clone)]
struct Directory {
    id: String,
    element: EditableTreeItem,
}

impl Default for EditableTree {
    fn default() -> Self {
        EditableTree::new(StrategyCatalog::default())
    }
}

impl EditableTree {
    pub fn new(strategy_catalog: StrategyCatalog) -> Self {
        EditableTree {
            items: vec![],
            strategy_catalog,
        }
    }

    fn add_directory(&mut self) {
        let new_directory = Directory {
            id: random_string(10),
            element: EditableTreeItem::new(self.strategy_catalog.clone()),
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

    pub fn view(&self) -> Element<'_, TreeMessage> {
        let columns = self
            .items
            .iter()
            .map(|dir: &Directory| -> Element<'_, TreeMessage> {
                dir.element
                    .view()
                    .map(move |item_message| TreeMessage::ItemEvent(dir.id.clone(), item_message))
            });

        let add_directory_btn: Element<'_, TreeMessage> = primary_button("Add Directory")
            .on_press(TreeMessage::AddEmptyItem)
            .width(Length::Fill)
            .into();

        column(columns.into_iter())
            .push(add_directory_btn)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }

    pub fn update(&mut self, message: TreeMessage) {
        match message {
            TreeMessage::AddEmptyItem => self.add_directory(),
            TreeMessage::ItemEvent(id, item_message) => {
                // react on the parent side
                match item_message.clone() {
                    TreeItemMessage::DirectoryRemoved => self.remove_item(id.clone()),
                    TreeItemMessage::MoveDirectory(movement) => {
                        self.move_item(id.clone(), movement);
                    }
                    _ => (),
                }

                // Some of the operation made above could have destroyed the item
                if let Some(directory) = self.items.iter_mut().find(|item| item.id == id) {
                    directory.element.update(item_message);
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
