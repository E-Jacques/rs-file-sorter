use iced::{
    border::Radius,
    widget::{button, column, container, row, text_input},
    Border, Color, Element, Length,
};

use crate::{
    core::sorting_strategy::SortingStrategy,
    sorting_strategies::{MONTH_SORTING_STRATEGY, YEAR_SORTING_STRATEGY},
    utils::string_manipulator::random_string,
};

const STRATEGIES_LIST: &[SortingStrategy<'static>] =
    &[MONTH_SORTING_STRATEGY, YEAR_SORTING_STRATEGY];

#[derive(Debug, Clone)]
pub struct EditableFileTree {
    directories: Vec<Directory>,
}

#[derive(Debug, Clone)]
struct Directory {
    id: String,
    name: String,
    strategy: Option<&'static SortingStrategy<'static>>,
}

#[derive(Debug, Clone)]
pub enum DirectoryMovement {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum Message {
    DirectoryAdded,
    DirectoryRemoved(String),
    StrategyChanged(String, String),
    MoveDirectory(String, DirectoryMovement),
}

impl Default for EditableFileTree {
    fn default() -> Self {
        EditableFileTree::new()
    }
}

impl EditableFileTree {
    fn new() -> Self {
        EditableFileTree {
            directories: vec![],
        }
    }

    fn add_directory(&mut self) {
        let new_directory = Directory {
            id: random_string(10),
            name: String::from(""),
            strategy: None,
        };
        self.directories.push(new_directory);
    }

    fn remove_directory(&mut self, id: String) {
        // Remove directory logic
        if let Some(index) = self.directories.iter().position(|dir| dir.id == id) {
            self.directories.remove(index);
        } else {
            println!("Directory with id {} not found", id);
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let columns = self
            .directories
            .iter()
            .map(|dir: &Directory| -> Element<'_, Message> {
                let dir_id = dir.id.clone();
                let delete_btn: Element<'_, Message> = button("delete")
                    .on_press(Message::DirectoryRemoved(dir_id.clone()))
                    .into();
                let up_btn: Element<'_, Message> = button("up")
                    .on_press(Message::MoveDirectory(
                        dir_id.clone(),
                        DirectoryMovement::Up,
                    ))
                    .into();
                let down_btn: Element<'_, Message> = button("down")
                    .on_press(Message::MoveDirectory(
                        dir_id.clone(),
                        DirectoryMovement::Down,
                    ))
                    .into();

                let strategy_name_input = text_input("Strategy", &dir.name.as_str())
                    .on_input(move |new_name| Message::StrategyChanged(dir_id.clone(), new_name));

                row![strategy_name_input, delete_btn, up_btn, down_btn]
                    .spacing(10)
                    .width(Length::Fill)
                    .into()
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
            Message::DirectoryRemoved(id) => self.remove_directory(id),
            Message::MoveDirectory(id, movement) => {
                self.move_directory(id, movement);
            }
            Message::StrategyChanged(id, name) => {
                self.change_directory_name(id, name);
            }
        }
    }

    fn change_directory_name(&mut self, id: String, name: String) {
        if let Some(dir) = self.directories.iter_mut().find(|dir| dir.id == id) {
            dir.name = name.clone();
            if let Some(strategy) = STRATEGIES_LIST.iter().find(|s| s.name == name) {
                dir.strategy = Some(strategy);
            } else {
                println!("Strategy already set for directory {}", dir.id);
            }
        } else {
            println!("Directory with id {} not found", id);
        }
    }

    fn move_directory(&mut self, id: String, movement: DirectoryMovement) {
        let index = self.directories.iter().position(|dir| dir.id == id);
        if let Some(index) = index {
            match movement {
                DirectoryMovement::Up => {
                    if index > 0 {
                        self.directories.swap(index, index - 1);
                    }
                }
                DirectoryMovement::Down => {
                    if index < self.directories.len() - 1 {
                        self.directories.swap(index, index + 1);
                    }
                }
            }
        }
    }

    pub fn get_sorting_strategies<'a>(&self) -> Vec<&'a SortingStrategy<'a>> {
        self.directories
            .iter()
            .filter_map(|dir| dir.strategy)
            .collect()
    }
}
