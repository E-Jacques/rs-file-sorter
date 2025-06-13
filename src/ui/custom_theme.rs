use iced::{border::Radius, theme::Palette, Border, Color, Theme};

use crate::ui::file_sorter_app::FileSorterApp;

pub fn theme(_: &FileSorterApp) -> Theme {
    Theme::custom(
        String::from("FileSorterTheme"),
        Palette {
            background: Color::WHITE,
            text: Color::from_rgb8(24, 24, 24),
            primary: Color::from_rgb8(42, 91, 255),
            success: Color::from_rgb8(230, 230, 230),
            danger: Color::from_rgb8(255, 0, 0),
        },
    )
}

pub fn border_style() -> Border {
    Border {
        color: Color::from_rgb8(230, 230, 230),
        width: 1.0,
        radius: Radius::new(4.0),
    }
}

pub struct ButtonPrimary {}

impl ButtonPrimary {
    pub fn style(
        theme: &Theme,
        status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let mut style = iced::widget::button::primary(theme, status);
        style.text_color = palette.primary.base.text;

        style
    }
}

pub struct ButtonSecondary {}

impl ButtonSecondary {
    pub fn style(
        theme: &Theme,
        status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        iced::widget::button::secondary(theme, status)
    }
}

pub struct TextInput {}

impl TextInput {
    pub fn style(
        theme: &Theme,
        status: iced::widget::text_input::Status,
    ) -> iced::widget::text_input::Style {
        let mut style = iced::widget::text_input::default(theme, status);
        style.border = border_style();

        style
    }
}
