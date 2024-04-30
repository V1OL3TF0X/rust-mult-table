use anyhow::Result;
use iced::{
    widget::{text, Column, Row, Text},
    Element,
};
use std::{iter::Take, path::Path, vec::IntoIter};

use crate::{
    components::equation::EqData,
    data::{consts::CELL_N, user::ScoreWithEq},
};

pub fn centered_text<'a, Renderer: iced::advanced::text::Renderer>(
    content: impl ToString,
) -> Text<'a, Renderer>
where
    Renderer::Theme: iced::widget::text::StyleSheet,
{
    text(content)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
}

pub fn extend_col<'a, Message, Renderer>(
    col: Column<'a, Message, Renderer>,
    iter: impl IntoIterator<Item = impl Into<Element<'a, Message, Renderer>>>,
) -> Column<'a, Message, Renderer> {
    iter.into_iter().fold(col, |acc, child| acc.push(child))
}

pub fn extend_row<'a, Message, Renderer>(
    row: Row<'a, Message, Renderer>,
    iter: impl IntoIterator<Item = impl Into<Element<'a, Message, Renderer>>>,
) -> Row<'a, Message, Renderer> {
    iter.into_iter().fold(row, |acc, child| acc.push(child))
}

mod cell {}

pub fn get_n(scores: &mut Take<IntoIter<ScoreWithEq>>) -> Option<[EqData; CELL_N]> {
    let v: Vec<(u32, u32)> = scores.take(CELL_N).map(|s| s.into()).collect();
    if v.len() == CELL_N {
        Some(
            v.into_iter()
                .map(|e| EqData::new(Some(e)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    } else {
        None
    }
}

pub fn make_nxn_mat<T: Default + std::fmt::Debug>() -> [[T; CELL_N]; CELL_N] {
    (0..CELL_N)
        .map(|_| {
            TryInto::<[T; CELL_N]>::try_into((0..CELL_N).map(|_| T::default()).collect::<Vec<_>>())
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub async fn load_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    Ok(tokio::fs::read(path).await?)
}

pub fn convert_to_msg<T, E, Message>(
    on_success: impl FnOnce(T) -> Message,
    on_error: impl FnOnce(E) -> Message,
) -> impl FnOnce(Result<T, E>) -> Message {
    |res: Result<T, E>| match res {
        Ok(r) => on_success(r),
        Err(e) => on_error(e),
    }
}

pub fn get_file_path(name: &str) -> std::path::PathBuf {
    let mut path = crate::data::consts::app_dir();
    path.push(format!("{name}.ron"));
    path
}
