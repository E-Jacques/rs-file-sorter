use iced::widget::{column, combo_box, row};

use crate::ui::template::template::Template;

pub struct TemplateManager {
    state: combo_box::State<String>,
    selected_template: Option<Template>,
    new_template_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameUpdate(String),
    SaveTemplate,
    LoadTemplate(String),
}

pub enum Event {
    SaveTemplate(String),
    LoadTemplate(String),
}

impl TemplateManager {
    pub fn new(templates: Vec<Template>) -> Self {
        TemplateManager {
            state: combo_box::State::new(templates.iter().map(|t| t.name.clone()).collect()),
            new_template_name: String::new(),
            selected_template: None,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        column![
            combo_box(
                &self.state,
                "Select Template",
                self.selected_template.as_ref().map(|t| &t.name),
                |name| Message::LoadTemplate(name),
            ),
            row![
                iced::widget::text_input("Template Name", &self.new_template_name)
                    .on_input(Message::NameUpdate),
                crate::ui::widget::button::primary_button::primary_button("save template")
                    .on_press(Message::SaveTemplate)
            ]
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        let mut maybe_event = None;

        match message {
            Message::SaveTemplate => {
                maybe_event = Some(Event::SaveTemplate(self.new_template_name.clone()));
            }
            Message::NameUpdate(value) => self.new_template_name = value,
            Message::LoadTemplate(name) => {
                self.new_template_name = name.clone();
                maybe_event = Some(Event::LoadTemplate(name));
            }
        }
        maybe_event
    }
}
