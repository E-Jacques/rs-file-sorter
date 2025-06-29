use iced::{widget::row, Element};

use crate::{
    core::sorter::FullSorterReport,
    ui::{
        screen::{
            sorter_form::{self, SortPayload, SorterForm},
            tree_preview::{self, TreePreview},
        },
        widget::alert::AlertSeverity,
    },
};

pub struct FileSorterApp {
    sorter_form: SorterForm,
    tree_preview: Option<TreePreview>,
    pipeline: Option<crate::core::SortPipeline>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TreePreviewMessage(tree_preview::Message),
    SorterFormMessage(sorter_form::Message),
}

#[derive(Debug, Clone)]
pub enum LogMessage {
    Warning(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum EventWrapper {
    SorterFormEvent(sorter_form::Event),
    TreePreviewEvent(tree_preview::Event),
}

impl Into<AlertSeverity> for &LogMessage {
    fn into(self) -> AlertSeverity {
        match self {
            LogMessage::Warning(_) => AlertSeverity::Warning,
            LogMessage::Error(_) => AlertSeverity::Error,
        }
    }
}

impl Default for FileSorterApp {
    fn default() -> Self {
        FileSorterApp::new()
    }
}

impl FileSorterApp {
    pub fn new() -> Self {
        FileSorterApp {
            sorter_form: SorterForm::default(),
            tree_preview: None,
            pipeline: None,
        }
    }

    fn sort(&mut self, sort_payload: SortPayload) {
        let mut log_messages: Vec<LogMessage> = vec![];

        self.pipeline = Some(crate::core::SortPipeline::new(
            sort_payload.input,
            sort_payload.output,
            sort_payload.sorting_strategies,
            sort_payload.options.clone(),
        ));
        let pipeline = self.pipeline.as_mut().unwrap();
        match pipeline.process() {
            Err(e) => {
                log_messages.push(LogMessage::Error(e.to_string()));
            }
            Ok(Some(reports)) => {
                if pipeline.has_next() {
                    self.tree_preview = Some(TreePreview::new(reports.clone()));
                }

                self.handle_report(reports);
            }
            Ok(None) => {
                log_messages.push(LogMessage::Error(
                    "Pipeline didn't reach expected step".to_string(),
                ));
            }
        }

        self.sorter_form.set_log_message(log_messages);
    }

    fn handle_report(&mut self, reports: FullSorterReport) {
        let mut log_messages: Vec<LogMessage> = vec![];

        for report in reports {
            match &report.result {
                Err(e) => {
                    log_messages.push(LogMessage::Warning(format!(
                        "Error processing file {}: {}",
                        report.input_filename.display(),
                        e
                    )));
                }
                _ => (),
            }
        }

        self.sorter_form.set_log_message(log_messages);
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![self.sorter_form.view().map(Message::SorterFormMessage)]
            .push_maybe(
                self.tree_preview
                    .as_ref()
                    .map(|tree_preview| tree_preview.view().map(Message::TreePreviewMessage)),
            )
            .into()
    }

    pub fn update(&mut self, message: Message) {
        let maybe_event: Option<EventWrapper> = match message {
            Message::TreePreviewMessage(tree_preview_message) => self
                .tree_preview
                .as_mut()
                .and_then(|tree_preview| tree_preview.update(tree_preview_message))
                .map(EventWrapper::TreePreviewEvent),
            Message::SorterFormMessage(sorter_form_message) => self
                .sorter_form
                .update(sorter_form_message)
                .map(EventWrapper::SorterFormEvent),
        };

        if let Some(event) = maybe_event {
            match event {
                EventWrapper::SorterFormEvent(sorter_form::Event::Sort(payload)) => {
                    self.sort(payload);
                }
                EventWrapper::TreePreviewEvent(tree_preview::Event::Apply) => {
                    if let Some(pipeline) = &mut self.pipeline {
                        let maybe_new_report = pipeline.process();

                        if let Ok(Some(new_report)) = maybe_new_report {
                            self.handle_report(new_report);
                        }
                    }
                }
            }
        }
    }
}
