use iced::{
    widget::{column, combo_box},
    Element,
};

use crate::{
    core::{sorting_strategy::SortingStrategy, strategy_parameter::StrategyParameter},
    sorting_strategies::strategy_catalog::StrategyCatalog,
};

use super::{
    default_editable_tree_item::DefaultEditableTreeItem,
    editable_tree::EditableTree,
    shared::{ItemMessage, StrategyOptions, TreeItem},
};

#[derive(Debug, Clone)]
pub struct NestedEditableTreeItem {
    selected_strategy: Option<String>,
    strategy_options: StrategyOptions,
    strategy_catalog: StrategyCatalog,
    editable_tree: EditableTree,
    header_element: DefaultEditableTreeItem,
}

impl NestedEditableTreeItem {
    pub fn new(strategy_catalog: StrategyCatalog) -> Self {
        let strategy_options = combo_box::State::new(strategy_catalog.get_names());
        NestedEditableTreeItem {
            selected_strategy: None,
            strategy_catalog: strategy_catalog.clone(),
            strategy_options,
            editable_tree: EditableTree::default(),
            header_element: DefaultEditableTreeItem::new(strategy_catalog),
        }
    }
}

impl TreeItem<ItemMessage> for NestedEditableTreeItem {
    fn view(&self) -> Element<'_, ItemMessage> {
        let header: Element<'_, ItemMessage> = self.header_element.view().into();
        let body: Element<'_, ItemMessage> = self
            .editable_tree
            .view()
            .map(move |message| ItemMessage::NestedEditableTreeMessage(Box::new(message)))
            .into();

        let spacing: f32 = 8.0;
        column!(header, body).spacing(spacing).into()
    }

    fn update(&mut self, item_message: ItemMessage) {
        self.header_element.update(item_message.clone());

        match item_message {
            ItemMessage::StrategyChanged(strategy) => {
                self.selected_strategy = Some(strategy);
            }
            ItemMessage::NestedEditableTreeMessage(nested_message) => {
                self.editable_tree.update(*nested_message.clone());
            }
            _ => (),
        }
    }

    fn box_clone(&self) -> Box<dyn TreeItem<ItemMessage>> {
        Box::new(NestedEditableTreeItem {
            selected_strategy: self.selected_strategy.clone(),
            strategy_options: self.strategy_options.clone(),
            strategy_catalog: self.strategy_catalog.clone(),
            editable_tree: self.editable_tree.clone(),
            header_element: self.header_element.clone(),
        })
    }

    fn get_sorting_strategy(&self) -> Option<SortingStrategy> {
        self.selected_strategy.clone().and_then(|strategy_name| {
            self.strategy_catalog
                .get_strategy(&strategy_name)
                .map(|mut strategy| {
                    strategy.add_parameter(
                        "strategies".to_string(),
                        StrategyParameter::Strategy(
                            self.editable_tree
                                .get_sorting_strategies()
                                .iter()
                                .cloned()
                                .map(Box::new)
                                .collect(),
                        ),
                    );
                    strategy
                })
        })
    }
}
