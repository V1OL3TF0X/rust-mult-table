use iced::{color, Element, Renderer};

use crate::helpers::centered_text;
use crate::styles::cell::{CellColor, CellStylesheet};

pub fn text_cell<'a, Message: 'a>(content: impl ToString) -> Cell<'a, Message> {
    cell(centered_text(content))
}

pub fn cell<'a, Message: 'a>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> Cell<'a, Message> {
    Cell::new(content.into())
}

pub struct Cell<'a, Message> {
    content: Element<'a, Message, Renderer>,
    color: CellColor,
    border: Option<iced::Color>,
    border_width: Option<f32>,
}

impl<'a, Message> Cell<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            content: content.into(),
            color: CellColor::White,
            border: Some(color!(0, 0, 0)),
            border_width: None,
        }
    }

    pub fn border(mut self, border: iced::Color) -> Self {
        self.border = Some(border);
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    pub fn border_maybe(mut self, border: Option<iced::Color>) -> Self {
        self.border = border;
        self
    }

    pub fn color(mut self, color: CellColor) -> Self {
        self.color = color;
        self
    }
    pub fn disabled(mut self, is_disabled: bool) -> Self {
        if is_disabled {
            self.color = CellColor::Grey;
            self.border = None;
        }
        self
    }
}

impl<'a, Message> From<Cell<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(cell: Cell<'a, Message>) -> Self {
        iced::widget::container(cell.content)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(35)
            .height(35)
            .style(iced::theme::Container::Custom(Box::new(
                CellStylesheet::new((&cell.color).into(), cell.border)
                    .border_width_maybe(cell.border_width),
            )))
            .into()
    }
}
