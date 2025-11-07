mod directory_input;
mod editable_tree;
mod option_form;
mod template_manager;

use iced::{
    widget::{column, container, scrollable, text, Column},
    Element, Length,
};

use crate::{
    sorting_strategies::all_catalog::all_catalog,
    ui::{
        custom_theme,
        file_sorter_app::LogMessage,
        shared,
        template::{self, template::Template},
        widget::{alert, button::primary_button},
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(directory_input::DirectoryInputMessage),
    OutputPathChanged(directory_input::DirectoryInputMessage),
    EditableFileTreeMessage(editable_tree::shared::TreeMessage),
    OptionFormMessage(option_form::Message),
    TemplateManagerMessage(template_manager::Message),
    Sort,
}

#[derive(Debug, Clone)]
pub enum Event {
    Sort(shared::sort_payload::SortPayload),
}

pub struct SorterForm {
    input_path: String,
    output_path: String,
    option_form: option_form::OptionForm,
    editable_file_tree: editable_tree::editable_tree::EditableTree,
    directory_input: directory_input::DirectoryInput,
    directory_output: directory_input::DirectoryInput,
    log_messages: Vec<LogMessage>,
    templates: Vec<Template>,
    template_manager: template_manager::TemplateManager,
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
            editable_file_tree: editable_tree::editable_tree::EditableTree::new(all_catalog()),
            directory_input: directory_input::DirectoryInput::new(None, "Input".to_string()),
            directory_output: directory_input::DirectoryInput::new(None, "Output".to_string()),
            log_messages: vec![],
            templates: template::manager::TemplateManager::list(),
            template_manager: template_manager::TemplateManager::new(
                crate::ui::template::manager::TemplateManager::list(),
            ),
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
            self.template_manager
                .view()
                .map(Message::TemplateManagerMessage),
            container(scrollable(Column::from_vec(alert_list).spacing(4)))
                .height(Length::Shrink)
                .max_height(200.0),
            container(
                column![
                    text("Input / Output").font({
                        let mut font = iced::Font::default();
                        font.weight = iced::font::Weight::Bold;
                        font
                    }),
                    input_path,
                    output_path
                ]
                .spacing(8.0)
            )
            .padding(16.0)
            .style(|_| {
                let mut style = container::Style::default();
                style.border = custom_theme::border_style();

                style
            }),
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
                return Some(Event::Sort(shared::sort_payload::SortPayload {
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
            Message::TemplateManagerMessage(message) => {
                let maybe_event = self.template_manager.update(message);

                if let Some(event) = maybe_event {
                    match event {
                        template_manager::Event::SaveTemplate(template_name) => {
                            let template = Template {
                                name: template_name,
                                strategies: self.editable_file_tree.payload(),
                                input: self.input_path.clone(),
                                output: self.output_path.clone(),
                                options: self.option_form.get_options(),
                            };

                            if let Err(err) =
                                crate::ui::template::manager::TemplateManager::save(template)
                            {
                                self.log_messages.push(LogMessage::Error(format!(
                                    "Failed to save template: {}",
                                    err
                                )));
                            }
                        }
                        template_manager::Event::LoadTemplate(template_name) => {
                            let maybe_template = self
                                .templates
                                .iter()
                                .find(|t| t.name == template_name)
                                .cloned();

                            if let Some(template) = maybe_template {
                                self.directory_input = directory_input::DirectoryInput::new(
                                    Some(template.input.clone()),
                                    "Input path".to_string(),
                                );
                                self.input_path = template.input.clone();

                                self.directory_output = directory_input::DirectoryInput::new(
                                    Some(template.output.clone()),
                                    "Output path".to_string(),
                                );
                                self.output_path = template.output.clone();

                                self.editable_file_tree =
                                    editable_tree::editable_tree::EditableTree::from(
                                        template.strategies,
                                    );
                                self.option_form = option_form::OptionForm::from(template.options);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
