mod file_sorter_app;
mod widgets;

pub fn run_app() {
    let _ = iced::run(
        "A cool counter",
        file_sorter_app::FileSorterApp::update,
        file_sorter_app::FileSorterApp::view,
    );
}
