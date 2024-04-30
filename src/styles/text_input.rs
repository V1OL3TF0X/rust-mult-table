use iced::{widget::text_input, Color};

use super::cell::CellColor;

pub struct CustomTextStyles {
    pub color: CellColor,
}

impl text_input::StyleSheet for CustomTextStyles {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: (&self.color).into(),
            border_radius: 5.0.into(),
            border_width: 2.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.weak.text,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: (&self.color).into(),
            border_radius: 5.0.into(),
            border_width: 2.0,
            border_color: palette.background.base.text,
            icon_color: palette.background.weak.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: (&self.color).into(),
            border_radius: 5.0.into(),
            border_width: 2.0,
            border_color: palette.primary.strong.color,
            icon_color: palette.background.weak.text,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        palette.background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.base.text
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().primary.weak.color
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();
        let background = if self.color == CellColor::White {
            palette.background.weak.color
        } else {
            (&self.color).into()
        }
        .into();
        text_input::Appearance {
            background,
            border_radius: 5.0.into(),
            border_width: 2.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.strong.color,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }
}
