use iced::{
    widget::{button, column, container, row, text},
    Element, Length,
};
use rfd::FileDialog;

use crate::{
    ui::{custom_theme, widget::icon},
    utils::string_manipulator::{elipsis, ElispsisDirection},
};

const MAX_CHAR: usize = 35;

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

        column![
            text(self.label.clone()).size(12.0).width(Length::Fill),
            row![
                container(
                    text(elipsis(
                        self.path.clone().unwrap_or(" ".to_string()),
                        MAX_CHAR,
                        ElispsisDirection::Middle,
                    ))
                    .wrapping(text::Wrapping::None)
                )
                .height(Length::Shrink)
                .width(Length::Fill)
                .clip(true)
                .padding(4)
                .style(|_| {
                    let mut style = container::Style::default();
                    style.border = custom_theme::border_style();

                    style
                }),
                button
            ]
            .spacing(8)
            .align_y(iced::Alignment::End)
        ]
        .spacing(4.0)
        .into()
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
