use file_sorter_app::FileSorterApp;

mod custom_theme;
mod file_sorter_app;
mod screen;
mod settings;
mod shared;
mod template;
mod widget;

pub fn run_app() {
    let _ =
        iced::application::application("File sorter", FileSorterApp::update, FileSorterApp::view)
            .font(widget::icon::FONT)
            .theme(custom_theme::theme)
            .window(settings::settings())
            .run();
}
