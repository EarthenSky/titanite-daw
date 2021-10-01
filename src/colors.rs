// todo: define all gui colors here

use iced::Color;

use iced::{
    button, checkbox, container, progress_bar, radio, rule, scrollable,
    slider, text_input,
};

// ---------------------------------------------------------------------- //

#[macro_export]
macro_rules! color {
    ( $r:expr, $g:expr, $b:expr, $a:expr ) => {
        Color {r: $r, g: $g, b: $b, a: $a}
    };
}

// ---------------------------------------------------------------------- //

// TODO: make it so that we can load themes from files.
pub static Light2: Color = Color {r:0.85, g:0.95, b:0.85, a:1.0};
pub static Light1: Color = color!(0.7, 0.8, 0.7, 1.0);
pub static Grey: Color = color!(0.4, 0.5, 0.4, 1.0);
pub static Dark1: Color = color!(0.3, 0.4, 0.3, 1.0);
pub static Dark2: Color = color!(0.1, 0.2, 0.1, 1.0);
pub static DarkFull: Color = color!(0.0, 0.05, 0.0, 1.0);

pub static BrightA: Color = color!(0.9, 0.4, 0.2, 1.0);
pub static BrightB: Color = color!(0.6, 0.9, 0.4, 1.0);

// ---------------------------------------------------------------------- //

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Dark1.into(),
            text_color: Light2.into(),
            ..container::Style::default()
        }
    }
}

// ---------------------------------------------------------------------- //

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Grey.into(),
            text_color: Light2.into(),
            border_color: Light1.into(),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Grey.into(),
            text_color: Light2.into(),
            border_color: Light1.into(),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Dark2.into(),
            text_color: Light1.into(),
            border_color: Grey.into(),
            ..button::Style::default()
        }
    }
}

// ---------------------------------------------------------------------- //
