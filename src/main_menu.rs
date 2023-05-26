use crate::GameState;
use bevy::prelude::*;
use bevy_iced::{
    iced::{
        widget::{button, column, container, text},
        Alignment, Length,
    },
    IcedContext,
};

pub struct Config;

#[derive(Clone)]
pub enum Message {
    StartClicked,
}

impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_event::<Message>()
            .add_systems((view, update).in_set(OnUpdate(GameState::MainMenu)));
    }
}

fn view(mut ctx: IcedContext<Message>) {
    let title = text("Bevy Bowling").size(50.0);

    let start = button("Start Game").on_press(Message::StartClicked);

    let column = column(vec![title.into(), start.into()])
        .align_items(Alignment::Center)
        .spacing(20);

    let container = container(column)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

    ctx.display(container);
}

fn update(mut messages: EventReader<Message>, mut game_state: ResMut<NextState<GameState>>) {
    for msg in messages.iter() {
        match msg {
            Message::StartClicked => {
                game_state.set(GameState::Playing);
            }
        }
    }
}
