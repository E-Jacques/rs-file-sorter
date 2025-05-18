use iced::{
    widget::{button, column, container, row, text_input},
    Element, Length,
};
use rfd::FileDialog;

use crate::{
    core::{sorter, sorting_strategy::SortingStrategy},
    utils::logger::Logger,
};

use super::widgets::{editable_file_tree, icon};

#[derive(Default)]
pub struct FileSorterApp<'a> {
    input_path: String,
    output_path: String,
    sorting_strategies: Vec<&'a SortingStrategy<'a>>,
    editable_file_tree: editable_file_tree::EditableFileTree,
}

#[derive(Debug, Clone)]
pub enum PathType {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(String),
    OutputPathChanged(String),
    OpenDirectorySelector(PathType),
    EditableFileTreeMessage(editable_file_tree::Message),
    Sort,
}

impl<'a> FileSorterApp<'a> {
    fn sort(&self) {
        sorter::sorter(
            &self.input_path,
            &self.output_path,
            self.sorting_strategies.clone(),
            Logger::new("File sorter App", true),
            |old, new| {
                println!("Error renaming file from {} to {}", old, new);
            },
        );
    }

    pub fn view(&self) -> Element<'_, Message> {
        let input_path = row![
            text_input("Input to sort", &self.input_path).on_input(Message::InputPathChanged),
            button(icon::icon(icon::FOLDER_CLOSED))
                .on_press(Message::OpenDirectorySelector(PathType::Input))
        ]
        .spacing(8);
        let output_path = row![
            text_input("Output path", &self.output_path).on_input(Message::OutputPathChanged),
            button(icon::icon(icon::FOLDER_CLOSED))
                .on_press(Message::OpenDirectorySelector(PathType::Output))
        ]
        .spacing(8);

        let output_path_tree = self
            .editable_file_tree
            .view()
            .map(move |msg| Message::EditableFileTreeMessage(msg));

        let sort_button = button("Sort").on_press(Message::Sort).width(Length::Fill);

        let content = column![input_path, output_path, output_path_tree, sort_button]
            .padding(20)
            .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Sort => {
                self.sort();
            }
            Message::InputPathChanged(path) => {
                self.input_path = path;
            }
            Message::OutputPathChanged(path) => {
                self.output_path = path;
            }
            Message::EditableFileTreeMessage(m) => {
                self.editable_file_tree.update(m);
                self.sorting_strategies = self.editable_file_tree.get_sorting_strategies();
            }
            Message::OpenDirectorySelector(path_type) => {
                let files = FileDialog::new().set_directory("/").pick_folder();
                if let Some(path) = files {
                    let path = String::from(path.to_str().unwrap());
                    match path_type {
                        PathType::Input => {
                            self.input_path = path;
                        }
                        PathType::Output => {
                            self.output_path = path;
                        }
                    }
                }
            }
        }
    }
}
