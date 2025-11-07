use iced::{
    widget::{column, container},
    Length,
};

use crate::{core::options::SortOptions, ui::custom_theme};

#[derive(Debug, Clone)]
pub struct OptionForm {
    options: SortOptions,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateDryRun(bool),
    UpdateRootOnly(bool),
}

impl OptionForm {
    pub fn new() -> Self {
        OptionForm {
            options: SortOptions::default(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                iced::widget::text("Options").font(custom_theme::TextFont::bold()),
                iced::widget::checkbox("Dry run", self.options.dry_run)
                    .on_toggle(|checked| Message::UpdateDryRun(checked)),
                iced::widget::checkbox("At root level only", self.options.root_level_only)
                    .on_toggle(|checked| Message::UpdateRootOnly(checked))
            ]
            .spacing(16)
            .width(Length::Fill),
        )
        .padding(16.0)
        .style(|_| {
            let mut style = container::Style::default();
            style.border = custom_theme::border_style();

            style
        })
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateRootOnly(root_level_only) => {
                self.options.root_level_only = root_level_only
            }
            Message::UpdateDryRun(dry_run) => self.options.dry_run = dry_run,
        };
    }

    pub fn get_options(&self) -> SortOptions {
        self.options.clone()
    }
}

impl From<SortOptions> for OptionForm {
    fn from(options: SortOptions) -> Self {
        OptionForm { options }
    }
}
