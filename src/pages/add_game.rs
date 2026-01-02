use std::path::PathBuf;

use iced::{padding::{self, horizontal}, widget, Alignment, Color, Element, Length};

use crate::{models::game::Game, resources};

#[derive(Clone)]
pub enum Message {
    Save,
    GameNameChanged(String),
    CoverPathChanged(String),
    CoverPathDialogOpen,
    ExePathChanged(String),
    ExePathDialogOpen,
    WineprefixChanged(String),
    WineprefixDialogOpen,
}

pub enum Action {
    Save(Game),
    None
}

#[derive(Default, Clone)]
pub struct State {
    game_name: String,
    cover_path: String,
    exe_path: String,
    wineprefix: String,
}

impl State {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Save => { 
                return Action::Save(
                    Game { 
                        name: self.game_name.clone(), 
                        cover_path: PathBuf::from(self.cover_path.clone()), 
                        exe_path: PathBuf::from(self.exe_path.clone()), 
                        wine_prefix: PathBuf::from(self.wineprefix.clone())
                    }
                )
            },
            Message::GameNameChanged(content) => { self.game_name = content; },
            Message::CoverPathChanged(content) => { self.cover_path = content; },
            Message::ExePathChanged(content) => { self.exe_path = content; },
            Message::WineprefixChanged(content) => { self.wineprefix = content; },
            Message::CoverPathDialogOpen => {
                let file = rfd::FileDialog::new().pick_file();
                if let Some(path) = file {
                    self.cover_path = path.to_string_lossy().to_string();
                }
            },
            Message::ExePathDialogOpen => {
                let file = rfd::FileDialog::new().pick_file();
                if let Some(path) = file {
                    self.exe_path = path.to_string_lossy().to_string();
                }
            },
            Message::WineprefixDialogOpen => {
                let folder = rfd::FileDialog::new().pick_folder();
                if let Some(path) = folder {
                    self.wineprefix= path.to_string_lossy().to_string();
                }
            }
        } 
        Action::None
    }

    pub fn view<'a>(&self) -> iced::Element<'a, Message> {
        widget::container(
            widget::column![
                widget::space().height(20),
                form_row(
                    "Game name", 
                    widget::text_input("", &self.game_name).on_input(Message::GameNameChanged),
                    None
                ),
                form_row(
                    "Cover path", 
                    widget::text_input("", &self.cover_path).on_input(Message::CoverPathChanged),
                    Some(
                        subtle_icon_button(
                            resources::FILE_OPEN.clone(), 
                            20, 20, Color::WHITE,
                            Message::CoverPathDialogOpen
                        )    
                    )
                ),
                form_row(
                    "Executable path", 
                    widget::text_input("", &self.exe_path).on_input(Message::ExePathChanged),
                    Some(
                        subtle_icon_button(
                            resources::FILE_OPEN.clone(), 
                            20, 20, Color::WHITE,
                            Message::ExePathDialogOpen
                        )    
                    )
                ),
                form_row(
                    "Wineprefix", 
                    widget::text_input("", &self.wineprefix).on_input(Message::WineprefixChanged),
                    Some(
                        subtle_icon_button(
                            resources::FOLDER_OPEN.clone(), 
                            20, 20, Color::WHITE,
                            Message::WineprefixDialogOpen
                        )    
                    )
                ),
                widget::space().height(40),
                widget::row![
                    widget::space().width(Length::Fill),
                    widget::button(widget::text("Save"))
                    .padding(horizontal(32).vertical(6))
                    .on_press(Message::Save),
                    widget::space().width(Length::Fill),
                ]
                .width(Length::Fill)
            ].spacing(16)
        ).padding(padding::horizontal(32))
        .into()
    }   
}

fn subtle_icon_button<'a>(
    handle: impl Into<widget::svg::Handle>, 
    width: impl Into<Length>,
    height: impl Into<Length>,
    color: Color,
    on_press: Message
) -> widget::Button<'a, Message> {
    widget::button(
        widget::svg(handle)
        .width(width).height(height)
        .style(move |_, _| { 
            widget::svg::Style {
                color: Some(color)
            }
        })
    )
    .style(|theme, status| widget::button::subtle(theme, status))
    .on_press(on_press)
}

fn form_row<'a>(
    label: &'a str, 
    input: widget::TextInput<'a, Message>, 
    button: Option<widget::Button<'a, Message>>
) -> widget::Row<'a, Message> {
    widget::row![
        widget::text(label)
            .width(Length::Fixed(128.0))
            .align_x(widget::text::Alignment::Left),
        input.width(Length::Fill).size(14).line_height(1.5),
        if let Some(button) = button {
            Into::<Element<'_, Message>>::into(button)
        } else { widget::space().into() }
    ]
    .spacing(12)
    .align_y(Alignment::Center)
}


