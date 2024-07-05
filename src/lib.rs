mod blocks;
mod loading;

#[cfg(target_family = "wasm")]
mod web;
use bevy::asset::AssetMetaCheck;
use bevy::render::camera::ScalingMode;
use bevy_pancam::{PanCam, PanCamPlugin};
use blocks::BlockPlugin;
#[cfg(target_family = "wasm")]
use web::WebPlugin;

use crate::loading::LoadingPlugin;

use bevy::app::App;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Main,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        app.add_plugins(WebPlugin);

        app.init_state::<GameState>()
            .insert_resource(AssetMetaCheck::Never)
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BSim2".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
                PanCamPlugin,
                LoadingPlugin,
                BlockPlugin,
            ))
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 10.,
        min_height: 5.,
    };
    commands.spawn((camera, PanCam::default()));
}
