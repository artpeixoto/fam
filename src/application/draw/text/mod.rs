pub mod font_dims;
pub mod normal;
pub mod tiny;
pub mod title;

use itertools::Itertools;
use macroquad::{
    color::Color,
    text::{
        Font, TextDimensions, TextParams, draw_multiline_text, draw_multiline_text_ex,
        draw_text_ex, measure_text,
    },
};
pub use normal::*;
pub use tiny::*;
pub use title::*;

use crate::application::draw::pos::ScreenUnit;
use crate::application::draw::{pos::Pos, text::font_dims::FontDimensions};

#[macro_export]
macro_rules! define_font {
    ($font_name: ident = {source: $source:literal, dims: $dims:expr}) => {
        #[allow(non_snake_case)]
        pub mod $font_name {
            use crate::application::draw::text::font_dims::FontDimensions;
            use ::macroquad::miniquad::FilterMode;
            use ::macroquad::prelude::Font;
            use ::macroquad::text::load_ttf_font_from_bytes;
            use ::std::sync::LazyLock;

            pub static FONT: LazyLock<Font> = LazyLock::new(|| {
                let mut font = load_ttf_font_from_bytes(include_bytes!($source)).unwrap();
                font.set_filter(FilterMode::Nearest);
                font
            });

            pub const DIMS: FontDimensions = $dims;
        }
    };
}

pub fn draw_text_pos(
    text: &str,
    top_left: Pos,
    text_style: TextStyle,
    scaling: u8,
    color: Color,
) -> TextDimensions {
    let font_dims = text_style.get_dims();
    let font = text_style.get_font();
    draw_text_ex(
        text,
        top_left.x as f32,
        (top_left.y + font_dims.top_height * scaling as i32) as f32,
        TextParams {
            font_size: font_dims.drawing_size * (scaling as u16),
            font_scale: 1.0,
            color,
            font: Some(font),
            ..Default::default()
        },
    )
}
pub fn draw_multiline_text_pos(
    text: &str,
    top_left: Pos,
    text_style: TextStyle,
    scaling: u8,
    color: Color,
) -> TextDimensions {
    let font_dims = text_style.get_dims();
    let font = text_style.get_font();
    draw_multiline_text_ex(
        text,
        top_left.x as f32,
        (top_left.y + font_dims.top_height * scaling as i32) as f32,
        None,
        TextParams {
            font_size: font_dims.drawing_size * (scaling as u16),
            font_scale: 1.0,
            color,
            font: Some(font),
            ..Default::default()
        },
    );
    measure_multiline_text(text, text_style, scaling)
}
pub fn measure_multiline_text(text: &str, text_style: TextStyle, scaling: u8) -> TextDimensions {
    let font_dims = text_style.get_dims();
    let font = text_style.get_font();
    let lines = text.lines().collect::<Vec<&str>>();
    let lines_dims = lines
        .iter()
        .map(|l| measure_text(l, Some(font), font_dims.drawing_size * scaling as u16, 1.0))
        .collect_vec();

    let max_width = lines_dims
        .iter()
        .map(|d| d.width as ScreenUnit)
        .max()
        .unwrap();
    let height = (lines.len() as ScreenUnit * font_dims.full_height()
        + (lines.len() - 1) as ScreenUnit * 1)
        * scaling as i32;

    TextDimensions {
        width: max_width as f32,
        height: height as f32,
        offset_y: 0.0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TextStyle {
    Tiny,
    #[default]
    Normal,
    Wide,
}

impl TextStyle {
    pub fn get_font(&self) -> &Font {
        match self {
            TextStyle::Tiny => &tiny_font::FONT,
            TextStyle::Normal => &normal_font::FONT,
            TextStyle::Wide => &title_font::FONT,
        }
    }
    pub const fn get_dims(&self) -> &FontDimensions {
        match self {
            TextStyle::Tiny => &tiny_font::DIMS,
            TextStyle::Normal => &normal_font::DIMS,
            TextStyle::Wide => &title_font::DIMS,
        }
    }
}
