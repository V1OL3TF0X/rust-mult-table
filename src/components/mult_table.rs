use std::sync::Arc;

use iced::{
    widget::{container, text, tooltip, Column, Row},
    Element, Renderer,
};

use crate::{
    components::cell::text_cell,
    data::{
        consts::{CELL_N, CELL_WIDTH, SPACING},
        user::User,
    },
    helpers::{extend_col, extend_row},
    styles::cell::{CellColor, CellStylesheet},
};

use super::equation::EqData;

pub enum Hidden {
    All,
    None,
    Specified([[bool; CELL_N]; CELL_N]),
}

pub fn mult_table<'a, Message: 'a>(
    user: &'a Option<Arc<User>>,
    hidden: &Hidden,
    selected: &[EqData; CELL_N],
) -> Element<'a, Message, Renderer> {
    let table_title = container(text("Tabliczka mnożenia")).center_x().width(
        (SPACING + CELL_WIDTH) * CELL_N as u16 /* 10 cells with equations + 10 spacings */ + CELL_WIDTH,
    );
    let label_row = extend_row(
        Row::new().push(text_cell('x').color(CellColor::DarkGrey)),
        (1..=CELL_N).map(|n| text_cell(n).color(CellColor::Grey)),
    )
    .spacing(SPACING);
    let selected: Vec<_> = selected.iter().filter_map(|e| e.get_numbers()).collect();
    extend_col(
        Column::new().push(table_title).push(label_row),
        (0..CELL_N).map(|j| {
            let row_label = text_cell(j + 1).color(CellColor::Grey);
            let table_cells = (0..CELL_N).map(|i| -> Element<'a, Message, Renderer> {
                let should_hide = match hidden {
                    Hidden::All => true,
                    Hidden::None => false,
                    Hidden::Specified(s) => s[i][j],
                };
                let score = user.as_ref().map(|u| u.get_score(i, j));
                let mut cell = match &score {
                    Some(s) if !should_hide => {
                        text_cell((i + 1) * (j + 1)).color(s.as_ref().into())
                    }
                    _ => text_cell(""),
                };

                if selected.contains(&(i as u32 + 1, j as u32 + 1)) {
                    cell = cell.border(CellColor::Green.into()).border_width(5.0)
                }

                if should_hide || score.is_none() {
                    return cell.into();
                }

                tooltip(
                    cell,
                    score.unwrap().to_string(),
                    tooltip::Position::FollowCursor,
                )
                .style(iced::theme::Container::Custom(Box::new(
                    CellStylesheet::new(CellColor::White.into(), None),
                )))
                .gap(10)
                .padding(10)
                .into()
            });
            extend_row(Row::new().push(row_label), table_cells).spacing(SPACING)
        }),
    )
    .spacing(SPACING)
    .into()
}
