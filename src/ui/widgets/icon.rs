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
pub const FILE: &str = "\u{E810}";

pub const WARNING: &str = "\u{E805}";
pub const INFO: &str = "\u{E806}";
pub const ERROR: &str = "\u{E807}";

#[allow(dead_code)]
pub const LOADER: &str = "\u{E808}";
#[allow(dead_code)]
pub const GITHUB: &str = "\u{E809}";

pub fn icon(code: &str) -> Text<'_> {
    text(code).font(Font::with_name("rsfs"))
}
