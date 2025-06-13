use file_sorter_app::FileSorterApp;
use iced::Size;

mod custom_theme;
mod file_sorter_app;
mod widgets;

pub fn run_app() {
    let _ =
        iced::application::application("File sorter", FileSorterApp::update, FileSorterApp::view)
            .window_size(Size::new(400.0, 600.0))
            .font(widgets::icon::FONT)
            .theme(custom_theme::theme)
            .run();
}
