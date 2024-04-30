use std::{iter::Take, sync::Arc, vec::IntoIter};

use crate::{
    components::{
        center_on_window::center,
        equation::{equation, CheckState, EqData},
        menu::menu,
        mult_table::Hidden,
    },
    data::{
        consts::{CELL_N, CELL_WIDTH, SPACING},
        user::{ScoreWithEq, User},
        user_list::UserList,
    },
    helpers::{centered_text, convert_to_msg, extend_col, get_n},
};
use anyhow::{anyhow, Error};
use iced::{
    alignment, executor, font,
    keyboard::{self, KeyCode},
    subscription::{self, Subscription},
    widget::{button, column as col, container, focus_next, focus_previous, row, text, Column},
    Application, Command, Event, Length, Theme,
};
use rand::{seq::SliceRandom, thread_rng};

pub struct MultiplicationTableApp {
    user: Option<Arc<User>>,
    user_list: Option<Arc<UserList>>,
    equations: [EqData; CELL_N],
    state: State,
    show_table: Hidden,
    show_results: bool,
    error: Option<Arc<Error>>,
}

enum State {
    NoTest,
    TestInProgress {
        remaining: Take<IntoIter<ScoreWithEq>>,
    },
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::TestInProgress { .. }, Self::TestInProgress { .. })
                | (Self::NoTest, Self::NoTest)
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(usize, Option<u32>),
    Focus(usize, bool),
    StartTest,
    UserListLoaded(UserList),
    UserLoaded(Box<User>, bool),
    UserCreated(Box<User>),
    UserSelected(String),
    UserFileRenamed(String),
    SetError(Option<Arc<anyhow::Error>>),
    CreateUser(String),
    RenameCurrent(String),
    SyncUserList,
    CheckResults,
    ContinueTest,
    StartSet,
}

type Msg = <MultiplicationTableApp as Application>::Message;

impl MultiplicationTableApp {
    fn init_test(&mut self, size: usize) {
        // SAFETY - if we init the test, the user must be loaded
        let mut all_scores: Vec<_> = self.user.as_mut().unwrap().iter().collect();
        all_scores.shuffle(&mut thread_rng());
        all_scores.sort_unstable();
        let remaining = all_scores.into_iter().take(size);
        self.state = State::TestInProgress { remaining };
        self.show_table = Hidden::Specified([[true; CELL_N]; CELL_N])
    }

    fn next_set(&mut self) -> Command<Msg> {
        self.show_results = false;
        let mut command = Command::none();
        if let State::TestInProgress { remaining } = &mut self.state {
            if self.equations[0].correctness != CheckState::Unckecked {
                // SAFETY - if we continue the test, it had to be initialized
                {
                    // SAFETY - NOONE should have access to the user but us at this time, so we can safely mutate
                    let user_ref = Arc::<User>::get_mut(self.user.as_mut().unwrap()).unwrap();
                    for e in self.equations {
                        let eq = e.get_numbers().unwrap();
                        if let Hidden::Specified(s) = &mut self.show_table {
                            s[eq.0 as usize - 1][eq.1 as usize - 1] = false;
                        }
                        // SAFETY - as before
                        let s = user_ref
                            .get_mut_score(eq.0 as usize - 1, eq.1 as usize - 1)
                            .unwrap();
                        s.update(e.correctness == CheckState::Correct);
                    }
                }
                if !self.show_results {
                    command = Self::save_results(&self.user);
                }
            }
            if remaining.len() != 0 {
                let eq = get_n(remaining);
                if let Some(e) = eq {
                    self.equations = e;
                    return command;
                }
            }
            self.equations = [EqData::new(None); CELL_N];
            self.show_table = Hidden::None;
            self.state = State::NoTest;
        }
        command
    }

    fn update_input(&mut self, index: usize, v: Option<u32>) -> Command<Msg> {
        self.equations[index].correctness = if let Some(c) = v {
            // SAFETY - if the value is in equations table, it 100% has the equation data
            let (n1, n2) = self.equations[index].get_numbers().unwrap();
            if n1 * n2 == c {
                CheckState::Correct
            } else {
                CheckState::Wrong
            }
        } else {
            CheckState::Unckecked
        };
        self.equations[index].value = v;
        Command::none()
    }

    fn update_focus(&mut self, index: usize, next: bool) -> Command<Msg> {
        if next {
            if index == CELL_N - 1 {
                Command::none()
            } else {
                focus_next()
            }
        } else {
            focus_previous()
        }
    }

    fn save_results(user: &Option<Arc<User>>) -> Command<Msg> {
        // SAFETY - if we want to save results, user HAS TO exist
        let user = user.as_ref().unwrap().clone();
        Command::perform(async move { user.update_file().await }, |r| {
            Msg::SetError(r.err())
        })
    }

    fn sync_user_list(&self) -> Command<Msg> {
        let ul = self.user_list.as_ref().unwrap().clone();
        Command::perform(async move { ul.save_to_file().await }, |r| {
            Msg::SetError(r.err())
        })
    }

    fn create_new_user(&mut self, user: String) -> Command<Msg> {
        // SAFETY - if we want to update the list, it's for sure loaded (it is loaded at the start of the program)
        Command::perform(
            async move { User::create_new(&(user)).await },
            convert_to_msg(
                |u| Msg::UserCreated(Box::new(u)),
                |e| Msg::SetError(Some(e)),
            ),
        )
    }

    fn rename_current_user(&mut self, new_user_name: String) -> Command<Msg> {
        // SAFETY - if we want to update the list, it's for sure loaded (it is loaded at the start of the program)
        let current_user = self.user.as_ref().unwrap().clone();
        Command::perform(
            async move { current_user.rename_user_file(new_user_name).await },
            convert_to_msg(Msg::UserFileRenamed, |e| Msg::SetError(Some(e))),
        )
    }
}

impl Application for MultiplicationTableApp {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default; //iced::futures::executor::ThreadPool;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                user: None,
                user_list: None,
                equations: [EqData::new(None); CELL_N],
                state: State::NoTest,
                show_table: Hidden::None,
                show_results: false,
                error: None,
            },
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(|r| {
                    Self::Message::SetError(
                        r.err()
                            .map(|_| Arc::new(anyhow!("Icon font didn't load correctly"))),
                    )
                }),
                Command::perform(UserList::load_from_file(), Self::Message::UserListLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        let name = if let Some(ref user) = self.user {
            user.name()
        } else {
            ""
        };
        format!("Multiplication table - {name}")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Input(i, v) => self.update_input(i, v),
            Message::Focus(i, next) => self.update_focus(i, next),
            Message::ContinueTest => self.next_set(),
            Message::StartTest => {
                self.init_test(CELL_N * CELL_N);
                self.next_set()
            }
            Message::StartSet => {
                self.init_test(CELL_N);
                self.next_set()
            }
            Message::CheckResults => {
                self.show_results = true;
                Self::save_results(&self.user)
            }
            Message::UserListLoaded(ul) => {
                self.user_list = Some(Arc::new(ul));
                let current = self.user_list.as_ref().unwrap().get_current();
                Command::perform(User::load_user(current.to_owned()), |u| {
                    Message::UserLoaded(u, false)
                })
            }
            Message::UserLoaded(u, should_sync) => {
                if let Some(ref mut ul) = self.user_list {
                    Arc::<UserList>::get_mut(ul)
                        .unwrap()
                        .switch_current(u.name().to_owned())
                }
                self.user = Some(Arc::new(*u));
                if should_sync {
                    Command::perform(async {}, |_| Msg::SyncUserList)
                } else {
                    Command::none()
                }
            }
            Message::UserCreated(u) => {
                let list = self.user_list.as_mut().unwrap();
                Arc::<UserList>::get_mut(list)
                    // SAFETY - NOONE should have access to the user but us at this time, so we can safely mutate
                    .unwrap()
                    .add_user(u.name());
                self.user = Some(Arc::new(*u));
                Command::perform(async {}, |_| Msg::SyncUserList)
            }
            Message::SetError(e) => {
                self.error = e;
                Command::none()
            }
            Message::CreateUser(u) => self.create_new_user(u),
            Message::UserSelected(u) => {
                Command::perform(User::load_user(u), |u| Message::UserLoaded(u, true))
            }
            Message::RenameCurrent(u) => self.rename_current_user(u),
            Message::SyncUserList => self.sync_user_list(),
            Message::UserFileRenamed(new_name) => {
                let list = self.user_list.as_mut().unwrap();
                Arc::<UserList>::get_mut(list)
                    // SAFETY - NOONE should have access to the user but us at this time, so we can safely mutate
                    .unwrap()
                    .update_current_user_after_rename(new_name.clone());
                Arc::<User>::get_mut(self.user.as_mut().unwrap())
                    // SAFETY - NOONE should have access to the user but us at this time, so we can safely mutate
                    .unwrap()
                    .set_name(new_name);
                let user = self.user.as_ref().unwrap().clone();
                Command::perform(async move { user.update_file().await }, |_| {
                    Msg::SyncUserList
                })
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let table_title = container(centered_text("Test"))
            .center_x()
            .width(EQUATION_WIDTH);

        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .on_press_maybe(on_press)
        };
        let has_all_answers = self
            .equations
            .iter()
            .all(|e| e.correctness != CheckState::Unckecked);

        let controls = row![
            if self.state == State::NoTest {
                button("Test", self.user.is_some().then_some(Message::StartTest))
            } else {
                button(
                    "Check",
                    (has_all_answers && !self.show_results).then_some(Message::CheckResults),
                )
            },
            if self.state == State::NoTest {
                button("1 Set", self.user.is_some().then_some(Message::StartSet))
            } else {
                button("Continue", has_all_answers.then_some(Message::ContinueTest))
            }
        ]
        .width(EQUATION_WIDTH)
        .height(35)
        .spacing(10);

        let table = center(
            row![
                extend_col(
                    Column::new().push(table_title),
                    self.equations.iter().enumerate().map(|(i, &e)| equation(
                        e,
                        self.show_results,
                        move |is_correct| Self::Message::Input(i, is_correct),
                        move || { Self::Message::Focus(i, true) }
                    ))
                )
                .width(Length::Shrink)
                .push(controls)
                .spacing(SPACING),
                crate::components::mult_table::mult_table(
                    &self.user,
                    &self.show_table,
                    &self.equations
                ),
            ]
            .spacing(30),
        );
        let menu = menu(self.user_list.as_ref().map(|ul| ul.get_all()))
            .on_create(Self::Message::CreateUser)
            .on_select(Self::Message::UserSelected)
            .on_rename_current(Self::Message::RenameCurrent);
        let mut layout = col![menu, table];
        if let Some(err) = &self.error {
            layout = layout.push(container(text(err)).center_x().width(Length::Fill))
        }
        layout.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: KeyCode::Tab,
                modifiers,
            }) => Some(Self::Message::Focus(0, !modifiers.shift())),
            _ => None,
        })
    }
}

const EQUATION_WIDTH: u16 = CELL_WIDTH * 3 + SPACING * 5 + 8;
