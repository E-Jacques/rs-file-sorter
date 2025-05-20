use iced::{
    widget::{button, text, Row},
    Element, Length,
};
use rfd::FileDialog;

use super::icon;
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
    placeholder: Option<String>,
}

impl Default for DirectoryInput {
    fn default() -> Self {
        DirectoryInput {
            path: None,
            placeholder: None,
        }
    }
}

impl DirectoryInput {
    pub fn new(path: Option<String>, placeholder: Option<String>) -> Self {
        DirectoryInput { path, placeholder }
    }

    pub fn view(&self) -> Element<'_, DirectoryInputMessage> {
        let displayed_path = self
            .path
            .clone()
            .unwrap_or(self.placeholder.clone().unwrap_or_default());
        let button =
            button(icon::icon(icon::FOLDER_CLOSED)).on_press(DirectoryInputMessage::OpenExplorer);
        Row::new()
            .push(text(displayed_path).width(Length::Fill))
            .push(button)
            .spacing(8)
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
