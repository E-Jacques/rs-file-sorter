use iced::{
    widget::{button, column, container, text_input},
    Element, Length,
};

use crate::{
    core::{sorter, sorting_strategy::SortingStrategy},
    utils::logger::Logger,
};

use super::widgets::editable_file_tree;

#[derive(Default)]
pub struct FileSorterApp<'a> {
    input_path: String,
    output_path: String,
    sorting_strategies: Vec<&'a SortingStrategy<'a>>,
    editable_file_tree: editable_file_tree::EditableFileTree,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(String),
    OutputPathChanged(String),
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
        let input_path =
            text_input("Input to sort", &self.input_path).on_input(Message::InputPathChanged);
        let output_path =
            text_input("Output path", &self.output_path).on_input(Message::OutputPathChanged);
        let output_path_tree = self
            .editable_file_tree
            .view()
            .map(move |msg| Message::EditableFileTreeMessage(msg));
        let button = button("Sort").on_press(Message::Sort).width(Length::Fill);
        let content = column![input_path, output_path, output_path_tree, button]
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
        }
    }
}
