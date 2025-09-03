mod directory_input;
mod editable_tree;
mod option_form;

use iced::{
    widget::{column, container, scrollable, Column},
    Element, Length,
};

use crate::{
    core::{options::SortOptions, strategy::Strategy},
    sorting_strategies::{
        manipulation_catalog::get_manipulation_catalog, metadata_catalog::get_metadata_catalog,
    },
    ui::{
        file_sorter_app::LogMessage,
        widget::{alert, button::primary_button},
    },
};

#[derive(Debug, Clone)]
pub struct SortPayload {
    pub input: String,
    pub output: String,
    pub strategies: Vec<Box<dyn Strategy>>,
    pub options: SortOptions,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(directory_input::DirectoryInputMessage),
    OutputPathChanged(directory_input::DirectoryInputMessage),
    EditableFileTreeMessage(editable_tree::shared::TreeMessage),
    OptionFormMessage(option_form::Message),
    Sort,
}

#[derive(Debug, Clone)]
pub enum Event {
    Sort(SortPayload),
}

pub struct SorterForm {
    input_path: String,
    output_path: String,
    option_form: option_form::OptionForm,
    editable_file_tree: editable_tree::editable_tree::EditableTree,
    directory_input: directory_input::DirectoryInput,
    directory_output: directory_input::DirectoryInput,
    log_messages: Vec<LogMessage>,
}

impl Default for SorterForm {
    fn default() -> Self {
        SorterForm::new()
    }
}

impl SorterForm {
    pub fn new() -> SorterForm {
        SorterForm {
            input_path: String::new(),
            output_path: String::new(),
            option_form: option_form::OptionForm::new(),
            editable_file_tree: editable_tree::editable_tree::EditableTree::new(
                get_metadata_catalog().with(&get_manipulation_catalog()),
            ),
            directory_input: directory_input::DirectoryInput::new(None, "Input path".to_string()),
            directory_output: directory_input::DirectoryInput::new(None, "Output path".to_string()),
            log_messages: vec![],
        }
    }

    pub fn set_log_message(&mut self, log_messages: Vec<LogMessage>) {
        self.log_messages = log_messages;
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
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

        let sort_button = primary_button::primary_button("Sort")
            .on_press(Message::Sort)
            .width(Length::Fill);

        let alert_list: Vec<Element<'_, Message>> = self
            .log_messages
            .iter()
            .map(|msg| match msg.clone() {
                LogMessage::Warning(text) | LogMessage::Error(text) => {
                    alert::alert(msg.into(), text.clone()).into()
                }
            })
            .collect();

        let content = column![
            container(scrollable(Column::from_vec(alert_list).spacing(4)))
                .height(Length::Shrink)
                .max_height(200.0),
            input_path,
            output_path,
            self.option_form.view().map(Message::OptionFormMessage),
            output_path_tree,
            sort_button
        ]
        .padding(20)
        .spacing(10);

        scrollable(content)
            .width(Length::Fixed(400.0))
            .height(Length::Shrink)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Sort => {
                let strategies = self.editable_file_tree.get_sorting_strategies();
                return Some(Event::Sort(SortPayload {
                    input: self.input_path.clone(),
                    output: self.output_path.clone(),
                    strategies,
                    options: self.option_form.get_options(),
                }));
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
            Message::OptionFormMessage(option_form_message) => {
                self.option_form.update(option_form_message);
            }
        }
        None
    }
}
