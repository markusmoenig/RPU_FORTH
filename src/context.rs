use crate::prelude::*;

pub struct Context {
    pub palette                 : Palette,

    pub font                    : Option<Font>,

    pub iso_state               : bool,
    pub render_state            : bool,
}

impl Context {
    pub fn new() -> Self {

        let mut palette = Palette::new();
        let mut font : Option<Font> = None;

        for file in Embedded::iter() {
            let name = file.as_ref();
            if name.starts_with("fonts/Roboto") {
                if let Some(font_bytes) = Embedded::get(name) {
                    if let Some(f) = Font::from_bytes(font_bytes.data, fontdue::FontSettings::default()).ok() {
                        font = Some(f);
                    }
                }
            } else
            if name == "aurora.txt" {
                if let Some(bytes) = Embedded::get(name) {
                    if let Some(string) = std::str::from_utf8(bytes.data.as_ref()).ok() {
                        palette.load_from_txt(string.to_string())
                    }
                }
            }
        }

        Self {
            palette,
            font,

            iso_state           : false,
            render_state        : false,
        }
    }
}