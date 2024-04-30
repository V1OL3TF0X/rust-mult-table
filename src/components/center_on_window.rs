use iced::{
    widget::{container, container::StyleSheet, Container},
    Element,
};

pub fn center<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> Container<'a, Message, Renderer>
where
    Renderer: iced::advanced::renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    container(content)
        .center_x()
        .center_y()
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
}
