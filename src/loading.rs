use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
    #[asset(path = "fonts/Silkscreen-Bold.ttf")]
    pub silk_bold: Handle<Font>,
    #[asset(path = "fonts/Silkscreen-Regular.ttf")]
    pub silk: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/menu.png")]
    pub main_menu_bg: Handle<Image>,
    #[asset(path = "textures/bevy.png")]
    pub player_texture: Handle<Image>,
    #[asset(path = "textures/gameover.png")]
    pub game_over_bg: Handle<Image>,
    #[asset(path = "textures/pl_front.png")]
    pub player_front: Handle<Image>,
    #[asset(path = "textures/pl_left.png")]
    pub player_left: Handle<Image>,
    #[asset(path = "textures/pl_right.png")]
    pub player_right: Handle<Image>,
    #[asset(path = "textures/bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "textures/lober.png")]
    pub enemy: Handle<Image>,
}
