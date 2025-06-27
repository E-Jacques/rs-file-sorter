use iced::{
    widget::{button, column, container, row, text},
    Element, Length,
};
use rfd::FileDialog;

use crate::ui::{custom_theme, widget::icon};

#[derive(Clone, Debug)]
pub enum DirectoryInputMessage {
    OpenExplorer,
}

#[derive(Clone, Debug)]
pub enum DirectoryInputEvent {
    SelectPath(String),
    FailSelectPath,
}

pub(crate) struct DirectoryInput {
    path: Option<String>,
    label: String,
}

impl Default for DirectoryInput {
    fn default() -> Self {
        DirectoryInput {
            path: None,
            label: String::default(),
        }
    }
}

impl DirectoryInput {
    pub fn new(path: Option<String>, label: String) -> Self {
        DirectoryInput { path, label }
    }

    pub fn view(&self) -> Element<'_, DirectoryInputMessage> {
        let button = button(icon::icon(icon::FOLDER_CLOSED))
            .on_press(DirectoryInputMessage::OpenExplorer)
            .style(custom_theme::ButtonPrimary::style);
        let header = row![text(self.label.clone()).width(Length::Fill), button]
            .spacing(8)
            .align_y(iced::Alignment::End);
        let body = container(text(self.path.clone().unwrap_or(" ".to_string())))
            .width(Length::Fill)
            .padding(4)
            .style(|_| {
                let mut style = container::Style::default();
                style.border = custom_theme::border_style();

                style
            });

        column![header, body].spacing(8).into()
    }

    pub fn update(&mut self, message: DirectoryInputMessage) -> DirectoryInputEvent {
        match message {
            DirectoryInputMessage::OpenExplorer => {
                let initial_path = self.path.clone().unwrap_or_default();
                let path = FileDialog::new().set_directory(&initial_path).pick_folder();

                if let Some(path) = path {
                    let path_str = path.to_owned().to_str().unwrap_or_default().to_string();
                    self.path = Some(path_str.clone());
                    DirectoryInputEvent::SelectPath(path_str.clone())
                } else {
                    DirectoryInputEvent::FailSelectPath
                }
            }
        }
    }
}
