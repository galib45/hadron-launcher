use iced::widget;

use crate::{models::game::Game, pages::{add_game, home}, widgets::app_bar};

pub enum Page {
    Home,
    AddGame(add_game::State),
    // EditGame(edit_game::State),
    // Settings(settings::State),
}

#[derive(Clone)]
pub enum Message {
    ToAddGame,
    AddGame(add_game::Message),
    LaunchGame(usize),
    ToHome,
}

pub struct App {
    games: Vec<Game>,
    current_page: Page,
}

impl App {
    pub fn new() -> Self {
        Self {
            games: vec![
                Game::default()
                    .with_name("Hollow Knight")
                    .with_cover_path("hollow-knight.jpg"),
                Game::default()
                    .with_name("Hollow Knight: Silksong")
                    .with_cover_path("hollow-knight-silksong.jpg"),
            ],
            current_page: Page::Home,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToHome => { self.current_page = Page::Home; }
            Message::ToAddGame => { self.current_page = Page::AddGame(add_game::State::default()); },
            Message::AddGame(message) => {
                if let Page::AddGame(state) = &mut self.current_page {
                    let action = state.update(message);
                    match action {
                        add_game::Action::Save(game) => {
                            self.games.push(game);
                            self.current_page = Page::Home;
                        }
                        add_game::Action::None => {}
                    }
                }
            }
            Message::LaunchGame(index) => { println!("launching game-{}", index) },
        } 
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let appbar = match &self.current_page {
            Page::Home => app_bar::view("Games", None),
            Page::AddGame(_) => app_bar::view("Add a game", Some(Message::ToHome)),
        };

        let content = match &self.current_page {
            Page::Home => home::view(&self.games),
            Page::AddGame(state) => state.view().map(move |message| Message::AddGame(message)),
        };

        let base = widget::column![appbar, content];

        base.into()
    }

    pub fn title(&self) -> String {
        match &self.current_page {
            Page::Home => "Hadron - Home",
            Page::AddGame(_) => "Hadron - Add a new game",
        }.into()
    }
}

