use iced::widget::text::Wrapping;
use iced::{padding, widget, Alignment, Color, ContentFit, Length};
use iced::widget::image;

use crate::app::Message;
use crate::models::game::Game;
use crate::resources;

pub fn view<'a>(games: &'a Vec<Game>) -> iced::Element<'a, Message> {
    widget::stack![
        // 1. Scrollable content ONLY
        widget::scrollable(
            widget::container(
                if games.is_empty() {
                    Into::<iced::Element<'_, Message>>::into(
                        widget::text("No Games Added")
                            .size(32).width(696)
                            .color(Color::from_rgba(0.5, 0.5, 0.5, 0.75))
                    )
                } else {
                    games_grid(games)
                        .columns(4).spacing(32)
                        .width(150 * 4 + 32 * 3)
                        .height(Length::Shrink)
                        .into()
                }
            )
            .padding(padding::horizontal(32))
        )
        .direction(iced::widget::scrollable::Direction::Both {
            horizontal: iced::widget::scrollable::Scrollbar::default(),
            vertical: iced::widget::scrollable::Scrollbar::default(),
        })
        .width(Length::Fill)
        .height(Length::Fill),

        // 2. Floating action button (LAST = on top)
        widget::float(
            widget::button(
                widget::row![
                    widget::svg(resources::SVG_ADD_2.clone())
                        .width(24).height(24)
                        .style(|_, _| { 
                            widget::svg::Style {
                                color: Some(Color::WHITE)
                            }
                        }),        
                    widget::text("Add Game")
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .on_press(Message::ToAddGame)
        )
        .translate(|original, viewport| {
            iced::Vector::new(
                viewport.width - original.width - 32.0,
                viewport.height - original.height - 128.0,
            )
        }),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn game_card(index: usize, name: &str, cover_path: impl Into<image::Handle>) -> iced::Element<'_, Message> {
    let base = widget::column![
        image(cover_path)
            .width(150).height(200)
            .content_fit(ContentFit::Fill)
            .border_radius(8),
        widget::text(name).wrapping(Wrapping::Word)
    ].width(150).height(280)
    .spacing(8)
    .align_x(Alignment::Center);

    let overlay = widget::container(widget::space())
        .width(150)
        .height(280)
        .style(|_theme| {
            widget::container::Style {
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
        .on_double_click(Message::LaunchGame(index))
        .into()
}

fn games_grid(games: &Vec<Game>) -> widget::Grid<'_, Message> {
    let mut grid = widget::grid::Grid::with_capacity(games.len());
    for (idx, game) in games.iter().enumerate() {
        grid = grid.push(game_card(idx, &game.name, game.cover_path.to_str().unwrap()));
    }
    grid
}
