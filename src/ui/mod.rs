mod file_sorter_app;
mod widgets;

pub fn run_app() {
    let _ = iced::application::application(
        "File sorter",
        file_sorter_app::FileSorterApp::update,
        file_sorter_app::FileSorterApp::view,
    )
    .font(widgets::icon::FONT)
    .run();
}
