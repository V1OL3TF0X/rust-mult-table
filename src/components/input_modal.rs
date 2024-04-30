use iced::{
    alignment::{self, Horizontal},
    widget::{container, text_input, Button, Row, Text},
    Element, Length, Renderer,
};

use iced_aw::Card;

use super::center_on_window::center;

pub fn input_modal<'a, Message>(
    underlay: impl Into<Element<'a, Message, Renderer>>,
    value: String,
    on_close: Message,
) -> Modal<'a, Message> {
    Modal::new(underlay, value, on_close)
}

pub struct Modal<'a, Message> {
    title: &'a str,
    ph: &'a str,
    underlay: Element<'a, Message, Renderer>,
    value: String,
    error: Option<&'a str>,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    on_close: Message,
    on_submit: Option<Message>,
    show_modal: bool,
}

impl<'a, Message> Modal<'a, Message> {
    pub fn new(
        underlay: impl Into<Element<'a, Message, Renderer>>,
        value: String,
        on_close: Message,
    ) -> Self {
        Self {
            title: "",
            ph: "",
            underlay: underlay.into(),
            show_modal: false,
            on_input: None,
            on_submit: None,
            error: None,
            on_close,
            value,
        }
    }

    pub fn on_input(mut self, handle: impl Fn(String) -> Message + 'static) -> Self {
        self.on_input = Some(Box::new(handle));
        self
    }
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }
    pub fn maybe_title(mut self, title: Option<&'a str>) -> Self {
        if let Some(title) = title {
            self.show_modal = true;
            self.title = title;
        }
        self
    }

    pub fn placeholder(mut self, ph: &'a str) -> Self {
        self.ph = ph;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
}

impl<'u, Message: 'u + Clone> From<Modal<'u, Message>> for Element<'u, Message, Renderer> {
    fn from(modal: Modal<'u, Message>) -> Self {
        let overlay = if modal.show_modal {
            let mut input = text_input(modal.ph, &modal.value);
            if let Some(h) = modal.on_input {
                input = input.on_input(h);
            }
            if let Some(h) = &modal.on_submit {
                input = input.on_submit(h.clone());
            }
            let input = container(input)
                .center_y()
                .width(Length::Fill)
                .padding([5, 0]);
            let content: Element<'u, Message, Renderer> = if let Some(err) = modal.error {
                iced::widget::column![input, container(err).center_x().width(Length::Fill)].into()
            } else {
                input.into()
            };
            Some(center(
                Card::new(
                    Text::new(modal.title),
                    content, //Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit​​, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead.")
                )
                .foot(
                    Row::new()
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)
                        .push(
                            Button::new(
                                Text::new("Cancel").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(modal.on_close.clone()),
                        )
                        .push(
                            Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                                .width(Length::Fill)
                                .on_press_maybe(modal.on_submit),
                        ),
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(modal.on_close.clone()),
            ))
        } else {
            None
        };

        iced_aw::modal(modal.underlay, overlay)
            .backdrop(modal.on_close.clone())
            .on_esc(modal.on_close)
            .align_y(alignment::Vertical::Top)
            .into()
    }
}
