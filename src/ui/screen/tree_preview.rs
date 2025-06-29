mod node;

use std::path::PathBuf;

use iced::{
    widget::{column, container, row, Column, Text},
    Length, Padding,
};

use crate::{
    core::sorter::FullSorterReport,
    ui::{
        custom_theme,
        widget::{alert::alert, icon},
    },
};

pub struct TreePreview {
    root_node: node::Node,
}

static DESCRIPTION:&'static str = "No file have been moved yet. Want you see is only a preview of want it may look likes after move files. Please press the 'Apply' button if you decide to proceed.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Apply,
}

#[derive(Debug, Clone)]
pub enum Event {
    Apply,
}

impl TreePreview {
    pub fn new(reports: FullSorterReport) -> Self {
        TreePreview {
            root_node: node::Node::from(
                reports
                    .iter()
                    .filter_map(move |report| report.result.as_ref().ok())
                    .cloned()
                    .collect::<Vec<PathBuf>>(),
            )
            .canonicalize(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let tree_representation: iced::Element<'_, Message> =
            self.render_node(&self.root_node, 0.0).into();

        container(
            column![
                row![
                    iced::widget::text("Sorter Report").width(Length::Fill),
                    iced::widget::button("Apply")
                        .on_press(Message::Apply)
                        .style(custom_theme::ButtonPrimary::style)
                ]
                .align_y(iced::Alignment::Center),
                alert(
                    crate::ui::widget::alert::AlertSeverity::Info,
                    DESCRIPTION.to_string()
                ),
                tree_representation
            ]
            .spacing(8),
        )
        .padding(16)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Apply => Some(Event::Apply),
        }
    }

    fn render_node<'a>(&self, node: &'a node::Node, indent: f32) -> Column<'a, Message> {
        let mut col = Column::new().spacing(2);

        let icon = match node.node_type {
            node::NodeType::File => icon::icon(icon::FILE),
            node::NodeType::Directory => icon::icon(icon::FOLDER_OPENED),
            node::NodeType::Other => Text::new("[?]"),
        };
        let left_padding: f32 = 24.0 * indent;
        // Add current node's name
        col = col.push(
            row![icon, Text::new(node.name.clone())]
                .spacing(4)
                .padding(Padding {
                    top: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: left_padding,
                }),
        );

        // Sort children for left-to-right consistency
        let mut sorted_keys: Vec<_> = node.children.keys().collect();
        sorted_keys.sort();

        // Recursively add child nodes
        for key in sorted_keys {
            let child = &node.children[key];
            col = col.push(self.render_node(child, indent + 1.0));
        }

        col.into()
    }
}
