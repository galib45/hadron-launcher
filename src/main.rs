use std::path::PathBuf;

use iced::widget::{self, space, Grid};
use iced::widget::{row, column, container};
use iced::widget::image::Handle;
use iced::widget::text::Wrapping; 
use iced::{Alignment, Color, ContentFit, Element, Length};


#[derive(Debug)]
struct Game {
    pub name: String,
    pub exe_path: PathBuf,
    pub cover_path: PathBuf,
}

impl Game {
    fn new(name: impl Into<String>, exe_path: impl Into<PathBuf>, cover_path: impl Into<PathBuf>) -> Self {
         Self {
             name: name.into(),
             exe_path: exe_path.into(),
             cover_path: cover_path.into(),
         }
    }
}

#[derive(Default, Debug)]
struct State {
    pub games: Vec<Game> 
}

impl State {
    fn load() -> Self {
        Self { 
            games: vec![
                Game::new("Hollow Knight", "", "hollow-knight.jpg"),
                Game::new("Hollow Knight: Silksong", "", "hollow-knight-silksong.jpg"),
            ]
        }
    }
}

#[derive(Clone)]
enum Message {
    GameSelected(usize),
    GameLaunching(usize),
    AddingGame,
}

fn update(_state: &mut State, message: Message) {
    match message {
        Message::GameSelected(index) => {
            
        },
        Message::GameLaunching(index) => {
            println!("Launching game-{}", index);
        },
        Message::AddingGame => {
            println!("adding game");
        }
    }
}

fn game_card(index: usize, name: &str, cover_path: impl Into<Handle>) -> Element<'_, Message> {
    let base = column![
        widget::image(cover_path)
            .width(150).height(200)
            .content_fit(ContentFit::Fill)
            .border_radius(8),
        widget::text(name).wrapping(Wrapping::Word)
    ].width(150).height(280)
    .spacing(8)
    .align_x(Alignment::Center);

    let overlay = container(space())
        .width(150)
        .height(280)
        .style(|_theme| {
            container::Style {
                background: Some(Color::from_rgba(0.3, 0.6, 1.0, 0.2).into()),
                border: iced::Border {
                    color: Color::from_rgb(0.3, 0.6, 1.0),
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        });

    let card = widget::hover(base, overlay);
    widget::mouse_area(card)
        .on_press(Message::GameSelected(index))
        .on_double_click(Message::GameLaunching(index))
        .into()
}

fn games_grid(games: &Vec<Game>) -> Grid<'_, Message> {
    let mut grid = widget::grid::Grid::with_capacity(games.len());
    for (idx, game) in games.iter().enumerate() {
        grid = grid.push(game_card(idx, &game.name, game.cover_path.to_str().unwrap()));
    }
    grid
}

fn view(state: &State) -> iced::Element<'_, Message> {
    widget::scrollable(
        container(
            column![
                row![
                    widget::text("Games").size(24),
                    space().width(Length::Fill),
                    widget::button(widget::text("+ Add Game"))
                    .on_press(Message::AddingGame)
                ],
                // widget::grid!(
                //     game_card(0, "Hollow Knight", "hollow-knight.jpg"),
                //     game_card(1, "Hollow Knight: Silksong", "hollow-knight-silksong.jpg"), 
                // )
                if state.games.is_empty() {
                    Into::<iced::Element<'_, Message>>::into(
                        widget::text("No Games Added")
                            .size(32).color(Color::from_rgba(0.5, 0.5, 0.5, 0.75))
                            .width(696)
                    )
                } else {
                    games_grid(&state.games)
                        .columns(4).spacing(32)
                        .width(150*4 + 32*3)
                        .height(Length::Shrink)
                        .into()
                },
            ].spacing(24)
        )
        .padding(32)
    )
    .direction(iced::widget::scrollable::Direction::Both {
        horizontal: iced::widget::scrollable::Scrollbar::default(),
        vertical: iced::widget::scrollable::Scrollbar::default(),
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into() 
}

fn main() -> iced::Result {
    let app = iced::application(State::load, update, view)
        .window_size((760, 570))
        .resizable(false)
        .title("Hadron");
    app.run()
}
