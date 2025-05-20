use file_sorter_app::FileSorterApp;

mod file_sorter_app;
mod widgets;

pub fn run_app() {
    let _ =
        iced::application::application("File sorter", FileSorterApp::update, FileSorterApp::view)
            .font(widgets::icon::FONT)
            .run();
}
