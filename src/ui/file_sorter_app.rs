use iced::widget::{button, column, Column};

#[derive(Default)]
pub struct FileSorterApp {
    input_path: String,
    output_path: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Sort,
}

impl FileSorterApp {
    pub fn view(&self) -> Column<Message> {
        column![button("+")]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Sort => println!("Sorting..."),
        }
    }
}
