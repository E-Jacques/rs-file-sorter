use iced::widget::button;

use crate::ui::{custom_theme, widgets::icon};

pub fn icon_button<'a, M>(icon: &'a str) -> button::Button<'a, M>
where
    M: Clone,
{
    button(icon::icon(icon)).style(custom_theme::ButtonSecondary::style)
}
