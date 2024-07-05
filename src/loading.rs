use crate::GameState;
use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Main)
                .load_collection::<GltfAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
    #[asset(path = "gates.glb")]
    pub gates: Handle<Gltf>,
}
