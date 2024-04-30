use iced::theme::TextInput;
use iced::widget::{component, container, row, text_input, Component};
use iced::{Element, Renderer};

use crate::components::cell::text_cell;
use crate::styles::{cell::CellColor, text_input::CustomTextStyles};

use crate::helpers::centered_text;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum CheckState {
    Unckecked,
    Correct,
    Wrong,
}

#[derive(Clone, Copy, Debug)]
pub struct EqData {
    numbers: Option<(u32, u32)>,
    pub value: Option<u32>,
    pub correctness: CheckState,
}

impl EqData {
    pub fn new(numbers: Option<(u32, u32)>) -> Self {
        Self {
            numbers,
            value: None,
            correctness: CheckState::Unckecked,
        }
    }

    pub fn get_numbers(&self) -> Option<(u32, u32)> {
        self.numbers
    }
}

impl Default for EqData {
    fn default() -> Self {
        Self::new(None)
    }
}

impl From<&CheckState> for CellColor {
    fn from(value: &CheckState) -> Self {
        match value {
            CheckState::Unckecked => Self::White,
            CheckState::Correct => Self::Green,
            CheckState::Wrong => Self::Red,
        }
    }
}

pub struct Equation<Message> {
    eq_data: EqData,
    show_checked: bool,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
    on_submit: Box<dyn Fn() -> Message>,
}

pub fn equation<Message>(
    eq_data: EqData,
    show_checked: bool,
    on_change: impl Fn(Option<u32>) -> Message + 'static,
    on_submit: impl Fn() -> Message + 'static,
) -> Equation<Message> {
    Equation::new(eq_data, show_checked, on_change, on_submit)
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    FocusNext,
}

impl<Message> Equation<Message> {
    pub fn new(
        eq_data: EqData,
        show_checked: bool,
        on_change: impl Fn(Option<u32>) -> Message + 'static,
        on_submit: impl Fn() -> Message + 'static,
    ) -> Self {
        Self {
            eq_data,
            show_checked,
            on_change: Box::new(on_change),
            on_submit: Box::new(on_submit),
        }
    }
}

impl<Message> Component<Message, Renderer> for Equation<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::InputChanged(value) => {
                let new_value = if value.is_empty() {
                    None
                } else {
                    let v = value.parse().ok();
                    if let Some(num) = &v {
                        if num <= &999 {
                            Some(*num)
                        } else {
                            self.eq_data.value
                        }
                    } else {
                        self.eq_data.value
                    }
                };
                if new_value != self.eq_data.value {
                    Some((self.on_change)(new_value))
                } else {
                    None
                }
            }
            Event::FocusNext => Some((self.on_submit)()),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        let numbers = self.eq_data.get_numbers();
        let (text1, text2) = numbers
            .map(|(n1, n2)| (n1.to_string(), n2.to_string()))
            .unwrap_or(("".into(), "".into()));
        let color = if self.show_checked {
            &self.eq_data.correctness
        } else {
            &CheckState::Unckecked
        }
        .into();
        let mut answer_input = text_input(
            "",
            self.eq_data
                .value
                .as_ref()
                .map(u32::to_string)
                .as_deref()
                .unwrap_or(""),
        )
        .padding([5, 4])
        .style(TextInput::Custom(Box::new(CustomTextStyles { color })))
        .width(35)
        .line_height(25.0 / 16.0);
        if numbers.is_some() && !self.show_checked {
            answer_input = answer_input
                .on_input(Event::InputChanged)
                .on_submit(Event::FocusNext);
        }

        container(
            row![
                text_cell(text1),
                container(centered_text('x')).height(35).center_y(),
                text_cell(text2),
                container(centered_text('=')).height(35).center_y(),
                answer_input,
            ]
            .spacing(10),
        )
        .into()
    }
}

impl<'a, Message> From<Equation<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(equation: Equation<Message>) -> Self {
        component(equation)
    }
}
