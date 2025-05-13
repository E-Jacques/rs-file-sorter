use file_sorter_app::FileSorterApp;

pub mod file_sorter_app;

pub fn run_app() {
    let _ = iced::run("A cool counter", FileSorterApp::update, FileSorterApp::view);
}
