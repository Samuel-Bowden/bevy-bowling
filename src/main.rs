use bevy::prelude::*;
use bevy_iced::IcedPlugin;

mod game;
mod main_menu;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Bowling".into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(IcedPlugin)
        .add_plugin(game::Config)
        .add_plugin(main_menu::Config)
        .run();
}
