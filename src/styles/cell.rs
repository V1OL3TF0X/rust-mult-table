use iced::color;

use crate::data::score::{Score, MAX_PERCENT};

#[derive(Debug, Clone, Copy, Default)]
pub struct CellStylesheet {
    pub(crate) background: Option<iced::Background>,
    pub(crate) border: Option<iced::Color>,
    pub(crate) border_width: f32,
}

impl CellStylesheet {
    pub fn new(color: iced::Color, border: Option<iced::Color>) -> Self {
        Self {
            background: Some(iced::Background::Color(color)),
            border,
            border_width: 2.0,
        }
    }

    pub fn border_width_maybe(mut self, opt: Option<f32>) -> Self {
        if let Some(border) = opt {
            self.border_width = border
        }
        self
    }
}

impl iced::widget::container::StyleSheet for CellStylesheet {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            text_color: None,
            background: self.background,
            border_radius: 5.0.into(),
            border_width: self.border_width,
            border_color: self
                .border
                .unwrap_or_else(|| style.extended_palette().background.weak.color),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum CellColor {
    Grey,
    DarkGrey,
    Green,
    Orange,
    Yellow,
    Red,
    White,
}

impl From<Score> for CellColor {
    fn from(value: Score) -> Self {
        From::<&Score>::from(&value)
    }
}

impl From<&Score> for CellColor {
    fn from(s: &Score) -> Self {
        match s.get_percentage() {
            Some(p) if p < MAX_PERCENT / 10 * 2 => Self::Red,
            Some(p) if p < MAX_PERCENT / 10 * 5 => Self::Orange,
            Some(p) if p < MAX_PERCENT / 10 * 7 => Self::Yellow,
            Some(p) if p <= MAX_PERCENT => Self::Green,
            Some(_) | None => Self::White,
        }
    }
}

impl From<&CellColor> for iced::Background {
    fn from(value: &CellColor) -> Self {
        iced::Background::Color(value.into())
    }
}

impl From<CellColor> for iced::Background {
    fn from(value: CellColor) -> Self {
        (&value).into()
    }
}

impl From<CellColor> for iced::Color {
    fn from(value: CellColor) -> Self {
        (&value).into()
    }
}

impl From<&CellColor> for iced::Color {
    fn from(val: &CellColor) -> Self {
        match val {
            CellColor::Red => color!(222, 50, 31),
            CellColor::Grey => color!(176, 176, 176),
            CellColor::White => color!(255, 255, 255),
            CellColor::Green => color!(59, 191, 89),
            CellColor::Orange => color!(245, 135, 32),
            CellColor::Yellow => color!(252, 223, 3),
            CellColor::DarkGrey => color!(119, 147, 158),
        }
    }
}
