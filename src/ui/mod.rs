use file_sorter_app::FileSorterApp;
use iced::Size;

mod file_sorter_app;
mod widgets;

pub fn run_app() {
    let _ =
        iced::application::application("File sorter", FileSorterApp::update, FileSorterApp::view)
            .window_size(Size::new(400.0, 600.0))
            .font(widgets::icon::FONT)
            .run();
}
