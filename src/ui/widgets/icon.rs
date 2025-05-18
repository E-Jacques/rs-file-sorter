use iced::{
    widget::{text, Text},
    Font,
};

pub const FONT: &[u8] = include_bytes!("../../../rsc/fonts/rsfs.ttf");

pub const DELETE: &str = "\u{E800}";
pub const ARROW_UP: &str = "\u{E801}";
pub const ARROW_DOWN: &str = "\u{E802}";
pub const FOLDER_CLOSED: &str = "\u{E803}";
pub const FOLDER_OPENED: &str = "\u{E804}";
pub const LOADER: &str = "\u{E805}";
pub const GITHUB: &str = "\u{E806}";

pub fn icon(code: &str) -> Text<'_> {
    text(code).font(Font::with_name("rsfs"))
}
