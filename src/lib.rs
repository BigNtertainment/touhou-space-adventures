mod actions;
mod audio;
mod background;
mod character;
mod enemy;
mod game_area;
mod loading;
mod menu;
mod player;
mod waves;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use background::BackgroundPlugin;
use enemy::EnemyPlugin;
use game_area::GameAreaPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

pub const WIDTH: f32 = 1280.;
pub const HEIGHT: f32 = 720.;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use waves::WavesPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(GameAreaPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(WavesPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(BackgroundPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
