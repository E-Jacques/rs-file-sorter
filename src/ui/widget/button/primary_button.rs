use iced::{widget, Element, Length, Renderer, Theme};

use crate::ui::custom_theme;

pub fn primary_button<'a, M>(
    content: impl Into<Element<'a, M, Theme, Renderer>>,
) -> widget::button::Button<'a, M>
where
    M: Clone + 'a,
{
    widget::button(
        widget::Column::new()
            .push(content)
            .width(Length::Fill)
            .align_x(iced::Alignment::Center),
    )
    .style(custom_theme::ButtonPrimary::style)
}
