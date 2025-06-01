use iced::{
    widget::{column, combo_box},
    Element,
};

use crate::{
    core::sorting_strategy::SortingStrategy, sorting_strategies::concat_strategy::concat_strategy,
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
    strategy_list: Vec<SortingStrategy>,
    editable_tree: EditableTree,
    header_element: DefaultEditableTreeItem,
}

impl NestedEditableTreeItem {
    pub fn new(strategy_list: Vec<SortingStrategy>) -> Self {
        let strategy_options = combo_box::State::new(
            strategy_list
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<String>>(),
        );
        NestedEditableTreeItem {
            selected_strategy: None,
            strategy_list: strategy_list.clone(),
            strategy_options,
            editable_tree: EditableTree::default(),
            header_element: DefaultEditableTreeItem::new(strategy_list),
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
            ItemMessage::DirectoryRemoved => (),
            ItemMessage::StrategyChanged(_) => (),
            ItemMessage::MoveDirectory(_) => (),
            ItemMessage::NestedEditableTreeMessage(nested_message) => {
                self.editable_tree.update(*nested_message.clone());
            }
        }
    }

    fn box_clone(&self) -> Box<dyn TreeItem<ItemMessage>> {
        Box::new(NestedEditableTreeItem {
            selected_strategy: self.selected_strategy.clone(),
            strategy_options: self.strategy_options.clone(),
            strategy_list: self.strategy_list.clone(),
            editable_tree: self.editable_tree.clone(),
            header_element: self.header_element.clone(),
        })
    }

    fn get_sorting_strategy(&self) -> Option<SortingStrategy> {
        match self.selected_strategy.clone() {
            None => None,
            Some(strategy_name) => {
                if strategy_name == "concat" {
                    let nested_sorting_strategies = self.editable_tree.get_sorting_strategies();
                    Some(concat_strategy(nested_sorting_strategies))
                } else {
                    None
                }
            }
        }
    }
}
