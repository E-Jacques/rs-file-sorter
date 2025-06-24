use iced::{
    widget::{column, container, scrollable, Column},
    Element, Length,
};

use crate::{
    core::{sorter, sorting_strategy::SortingStrategy},
    sorting_strategies::{
        manipulation_catalog::get_manipulation_catalog, metadata_catalog::get_metadata_catalog,
    },
    ui::widgets::{
        alert::{alert, AlertSeverity},
        buttons::primary_button::primary_button,
    },
};

use super::widgets::{directory_input, editable_tree};

pub struct FileSorterApp {
    input_path: String,
    output_path: String,
    sorting_strategies: Vec<SortingStrategy>,
    editable_file_tree: editable_tree::editable_tree::EditableTree,
    directory_input: directory_input::DirectoryInput,
    directory_output: directory_input::DirectoryInput,
    log_messages: Vec<LogMessage>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(directory_input::DirectoryInputMessage),
    OutputPathChanged(directory_input::DirectoryInputMessage),
    EditableFileTreeMessage(editable_tree::shared::TreeMessage),
    Sort,
}

#[derive(Debug, Clone)]
pub enum LogMessage {
    Warning(String),
    Error(String),
}

impl Into<AlertSeverity> for &LogMessage {
    fn into(self) -> AlertSeverity {
        match self {
            LogMessage::Warning(_) => AlertSeverity::Warning,
            LogMessage::Error(_) => AlertSeverity::Error,
        }
    }
}

impl Default for FileSorterApp {
    fn default() -> Self {
        FileSorterApp::new()
    }
}

impl FileSorterApp {
    pub fn new() -> Self {
        FileSorterApp {
            input_path: String::new(),
            output_path: String::new(),
            sorting_strategies: vec![],
            editable_file_tree: editable_tree::editable_tree::EditableTree::new(
                get_metadata_catalog().with(&get_manipulation_catalog()),
            ),
            directory_input: directory_input::DirectoryInput::new(None, "Input path".to_string()),
            directory_output: directory_input::DirectoryInput::new(None, "Output path".to_string()),
            log_messages: vec![],
        }
    }

    fn sort(&mut self) {
        self.log_messages.clear();
        match sorter::sorter(
            &self.input_path,
            &self.output_path,
            self.sorting_strategies.clone(),
        ) {
            Err(e) => {
                self.log_messages.push(LogMessage::Error(e.to_string()));
            }
            Ok(reports) => {
                for report in reports {
                    match report.result {
                        Err(e) => {
                            self.log_messages.push(LogMessage::Warning(format!(
                                "Error processing file {}: {}",
                                report.input_dir.display(),
                                e
                            )));
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let input_path = self
            .directory_input
            .view()
            .map(|msg| Message::InputPathChanged(msg));

        let output_path = self
            .directory_output
            .view()
            .map(|msg| Message::OutputPathChanged(msg));

        let output_path_tree = self
            .editable_file_tree
            .view()
            .map(|msg| Message::EditableFileTreeMessage(msg));

        let sort_button = primary_button("Sort")
            .on_press(Message::Sort)
            .width(Length::Fill);

        let alert_list: Vec<Element<'_, Message>> = self
            .log_messages
            .iter()
            .map(|msg| match msg.clone() {
                LogMessage::Warning(text) | LogMessage::Error(text) => {
                    alert(msg.into(), text.clone()).into()
                }
            })
            .collect();

        let content = column![
            container(scrollable(Column::from_vec(alert_list).spacing(4)))
                .height(Length::Shrink)
                .max_height(200.0),
            input_path,
            output_path,
            output_path_tree,
            sort_button
        ]
        .padding(20)
        .spacing(10);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Sort => {
                self.sorting_strategies = self.editable_file_tree.get_sorting_strategies();
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
            }
        }
    }
}
