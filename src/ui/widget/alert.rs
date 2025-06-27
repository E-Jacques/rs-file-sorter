use iced::{
    advanced::{text, Widget},
    widget::row,
};

use crate::ui::{
    custom_theme,
    widget::icon::{self, icon},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
}

impl Into<iced::Color> for AlertSeverity {
    fn into(self) -> iced::Color {
        match self {
            AlertSeverity::Info => iced::Color::from_rgb(0.2, 0.6, 1.0), // Blue
            AlertSeverity::Warning => iced::Color::from_rgb(1.0, 0.8, 0.0), // Yellow
            AlertSeverity::Error => iced::Color::from_rgb(1.0, 0.2, 0.2), // Red
        }
    }
}
pub struct Alert<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced::widget::container::Catalog,
    Renderer: text::Renderer,
{
    container: iced::widget::Container<'a, (), Theme, Renderer>,
    _phantom: std::marker::PhantomData<Message>,
}

impl<'a, Message> Alert<'a, Message> {
    pub fn new(severity: AlertSeverity, message: String) -> Self {
        let icon = match severity {
            AlertSeverity::Info => icon(icon::INFO),
            AlertSeverity::Warning => icon(icon::WARNING),
            AlertSeverity::Error => icon(icon::ERROR),
        };

        let container = iced::widget::container::Container::new(
            row![icon, iced::widget::text(message)]
                .spacing(10)
                .padding(10),
        )
        .style(move |_| {
            let mut style = iced::widget::container::Style::default();
            style.border = custom_theme::border_style();
            style.background = Some(iced::Background::Color(severity.into()));
            style.text_color = Some(iced::Color::WHITE);
            style
        });
        Alert::<Message> {
            container,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Alert<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: text::Renderer,
    Theme: iced::widget::container::Catalog + 'a,
{
    fn size(&self) -> iced::Size<iced::Length> {
        Widget::<(), Theme, Renderer>::size(&self.container)
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        Widget::<(), Theme, Renderer>::size_hint(&self.container)
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        Widget::<(), Theme, Renderer>::children(&self.container)
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        Widget::<(), Theme, Renderer>::layout(&self.container, tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        Widget::<(), Theme, Renderer>::draw(
            &self.container,
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.container.tag()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.container.state()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        self.container.diff(tree);
    }

    fn operate(
        &self,
        state: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.container.operate(state, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        _state: &mut iced::advanced::widget::Tree,
        _event: iced::Event,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        iced::advanced::graphics::core::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &iced::advanced::widget::Tree,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        iced::advanced::mouse::Interaction::None
    }
}

impl<'a, Message> From<Alert<'a, Message>> for iced::Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(alert: Alert<'a, Message>) -> Self {
        Self::new(alert)
    }
}

pub fn alert<'a, Message: Clone>(severity: AlertSeverity, message: String) -> Alert<'a, Message> {
    Alert::<Message>::new(severity, message)
}
