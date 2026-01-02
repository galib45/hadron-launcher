use iced::{widget, Alignment, Color, Length};

use crate::{app::Message, resources};

pub fn view<'a>(title: impl Into<String>, on_back: Option<Message>) -> iced::Element<'a, Message> {
    widget::container(
        widget::row![
            widget::text(title.into()).size(24),
            widget::space().width(Length::Fill),
            if let Some(message) = on_back {
                Into::<iced::Element<'_, Message>>::into(
                    widget::button(
                        widget::row![
                            widget::svg(resources::ARROW_BACK_IOS.clone())
                            .width(20).height(20)
                            .style(|_, _| { 
                                widget::svg::Style {
                                    color: Some(Color::WHITE)
                                }
                            }),
                            widget::text("Back")
                        ]
                        .align_y(Alignment::Center)
                    )
                    .style(|theme, status| widget::button::background(theme, status))
                    .on_press(message)
                )
            } else {
                Into::<iced::Element<'_, Message>>::into(
                    widget::space() 
                )
            }
        ]
    ).padding(32)
    .into()
}
