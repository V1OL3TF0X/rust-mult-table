use iced::{
    alignment,
    widget::{button, component, container, text, Component},
    Element, Length, Renderer,
};
use iced_aw::{helpers::menu_tree, menu_tree, MenuBar};

use crate::styles::menu_button::ButtonStyle;

pub struct Menu<'u, Message> {
    user_list: Option<&'u Vec<String>>,
    on_create: Option<Box<dyn Fn(String) -> Message>>,
    on_select: Option<Box<dyn Fn(String) -> Message>>,
    on_rename_current: Option<Box<dyn Fn(String) -> Message>>,
}

impl<'u, Message> Menu<'u, Message> {
    pub fn new(user_list: Option<&'u Vec<String>>) -> Self {
        Self {
            user_list,
            on_create: None,
            on_select: None,
            on_rename_current: None,
        }
    }

    pub fn on_create(mut self, handle: impl Fn(String) -> Message + 'static) -> Self {
        self.on_create = Some(Box::new(handle));
        self
    }

    pub fn on_select(mut self, handle: impl Fn(String) -> Message + 'static) -> Self {
        self.on_select = Some(Box::new(handle));
        self
    }

    pub fn on_rename_current(mut self, handle: impl Fn(String) -> Message + 'static) -> Self {
        self.on_rename_current = Some(Box::new(handle));
        self
    }
}

pub fn menu<Message>(user_list: Option<&'_ Vec<String>>) -> Menu<'_, Message> {
    Menu::new(user_list)
}

#[derive(Debug, Clone)]
pub enum Event<'u> {
    UserSelected(&'u str),
    OpenAddUserModal,
    OpenRenameModal,
    ModalInput(String),
    ModalSubmit,
    CloseModal,
    Noop,
}

pub struct State {
    modal_title: Option<&'static str>,
    input_value: String,
    error: Option<String>,
}

impl State {
    fn close(&mut self) {
        self.modal_title = None;
        self.input_value = "".into();
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            modal_title: None,
            error: None,
            input_value: "".into(),
        }
    }
}

impl<'u, Message> Component<Message, Renderer> for Menu<'u, Message> {
    type State = State;

    type Event = Event<'u>;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        let mut ev = None;
        match event {
            Event::UserSelected(u) => {
                ev = self
                    .on_select
                    .is_some()
                    .then(|| (self.on_select.as_ref().unwrap())(u.to_owned()))
            }
            Event::OpenAddUserModal => state.modal_title = Some("Add new user"),
            Event::OpenRenameModal => state.modal_title = Some("Rename current user"),
            Event::CloseModal => state.close(),
            Event::Noop => (),
            Event::ModalInput(s) => state.input_value = s,
            Event::ModalSubmit => {
                if self
                    .user_list
                    .unwrap()
                    .iter()
                    .any(|u| u == &state.input_value)
                {
                    state.error = Some("User with this name already exusts".into());
                } else {
                    let v = state.input_value.clone();
                    ev = if state.modal_title.unwrap().contains("Add") {
                        self.on_create
                            .is_some()
                            .then(|| (self.on_create.as_ref().unwrap())(v))
                    } else {
                        self.on_rename_current
                            .is_some()
                            .then(|| (self.on_rename_current.as_ref().unwrap())(v))
                    };
                    state.error = None;
                    state.close();
                }
            }
        }
        ev
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let choose_user_button = base_button("Choose user");
        let user_list_loaded = self.user_list.is_some();
        let first = if let Some(users) = self.user_list {
            let children = users
                .iter()
                .fold(Vec::with_capacity(users.len()), |mut c, u| {
                    c.push(item(u, Self::Event::UserSelected(u)));
                    c
                });
            menu_tree(
                // attach on_press to make the menu appear enabled
                choose_user_button.on_press(Self::Event::Noop),
                children,
            )
        } else {
            menu_tree!(choose_user_button)
        };
        let add_user_button = menu_tree!(base_button("add new user")
            .on_press_maybe(user_list_loaded.then_some(Event::OpenAddUserModal)));
        let rename_current_button = menu_tree!(base_button("Rename curent user")
            .on_press_maybe(user_list_loaded.then_some(Self::Event::OpenRenameModal)));
        let menu = MenuBar::new(vec![first, add_user_button, rename_current_button])
            .spacing(2.0)
            .bounds_expand(30)
            .cross_offset(16);
        let menu_row = iced::widget::row!(menu, iced::widget::horizontal_space(Length::Fill))
            .padding([2, 8])
            .align_items(alignment::Alignment::Center);
        super::input_modal::input_modal(
            container(menu_row).width(Length::Fill),
            state.input_value.clone(),
            Self::Event::CloseModal,
        )
        .maybe_title(state.modal_title)
        .on_input(Self::Event::ModalInput)
        .on_submit(Self::Event::ModalSubmit)
        .into()
    }
}

impl<'u, Message: 'u> From<Menu<'u, Message>> for Element<'u, Message, Renderer> {
    fn from(menu: Menu<'u, Message>) -> Self {
        component(menu)
    }
}

fn base_button<Message>(label: &str) -> button::Button<'_, Message, iced::Renderer> {
    button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
    .padding([4, 8])
}

fn item<'a, Message: Clone + 'a>(
    label: &'a str,
    msg: Message,
) -> iced_aw::MenuTree<'a, Message, Renderer> {
    menu_tree!(base_button(label)
        .on_press(msg)
        .width(Length::Fill)
        .height(Length::Fill))
}
