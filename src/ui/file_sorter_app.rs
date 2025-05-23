use iced::{
    widget::{button, column, container},
    Element, Length,
};

use crate::{
    core::{sorter, sorting_strategy::SortingStrategy},
    utils::logger::Logger,
};

use super::widgets::{directory_input, editable_file_tree};

pub struct FileSorterApp<'a> {
    input_path: String,
    output_path: String,
    sorting_strategies: Vec<&'a SortingStrategy<'a>>,
    editable_file_tree: editable_file_tree::EditableFileTree,
    directory_input: directory_input::DirectoryInput,
    directory_output: directory_input::DirectoryInput,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(directory_input::DirectoryInputMessage),
    OutputPathChanged(directory_input::DirectoryInputMessage),
    EditableFileTreeMessage(editable_file_tree::Message),
    Sort,
}

impl Default for FileSorterApp<'_> {
    fn default() -> Self {
        FileSorterApp::new()
    }
}

impl<'a> FileSorterApp<'a> {
    pub fn new() -> Self {
        FileSorterApp {
            input_path: String::new(),
            output_path: String::new(),
            sorting_strategies: vec![],
            editable_file_tree: editable_file_tree::EditableFileTree::default(),
            directory_input: directory_input::DirectoryInput::new(
                None,
                Some(String::from("Input path")),
            ),
            directory_output: directory_input::DirectoryInput::new(
                None,
                Some(String::from("Output path")),
            ),
        }
    }

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
        let input_path = self
            .directory_input
            .view()
            .map(move |msg| Message::InputPathChanged(msg));

        let output_path = self
            .directory_output
            .view()
            .map(move |msg| Message::OutputPathChanged(msg));

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
            Message::InputPathChanged(message) => {
                match self.directory_input.update(message.clone()) {
                    directory_input::DirectoryInputEvent::SelectPath(path) => {
                        self.input_path = path;
                    }
                    directory_input::DirectoryInputEvent::FailSelectPath => {}
                }
            }
            Message::OutputPathChanged(message) => {
                match self.directory_output.update(message.clone()) {
                    directory_input::DirectoryInputEvent::SelectPath(path) => {
                        self.output_path = path;
                    }
                    directory_input::DirectoryInputEvent::FailSelectPath => {}
                }
            }
            Message::EditableFileTreeMessage(m) => {
                self.editable_file_tree.update(m);
                self.sorting_strategies = self.editable_file_tree.get_sorting_strategies();
            }
        }
    }
}
