use bevy::gltf::{Gltf, GltfMesh};
use bevy::scene::SceneInstance;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::loading::GltfAssets;
use crate::GameState;

#[derive(Resource)]
pub struct BlockMaterials {
    red: Handle<ColorMaterial>,
    yellow: Handle<ColorMaterial>,
    green: Handle<ColorMaterial>,
    cyan: Handle<ColorMaterial>,
    blue: Handle<ColorMaterial>,
    magenta: Handle<ColorMaterial>,

    gray: Handle<ColorMaterial>,
}

impl BlockMaterials {
    pub fn new(materials: &mut ResMut<Assets<ColorMaterial>>) -> BlockMaterials {
        BlockMaterials {
            red: materials.add(Color::rgb(1., 0., 0.)),
            yellow: materials.add(Color::rgb(1., 1., 0.)),
            green: materials.add(Color::rgb(0., 1., 0.)),
            cyan: materials.add(Color::rgb(0., 1., 1.)),
            blue: materials.add(Color::rgb(0., 0., 1.)),
            magenta: materials.add(Color::rgb(1., 0., 1.)),
            gray: materials.add(Color::rgb(0.5, 0.5, 0.5)),
        }
    }
}

#[derive(Resource)]
pub struct BlockMeshes {
    and: Mesh2dHandle,
    or: Mesh2dHandle,
    xor: Mesh2dHandle,
    not: Mesh2dHandle,
}

impl BlockMeshes {
    pub fn new(
        gltfs: Res<GltfAssets>,
        assets_gltf: Res<Assets<Gltf>>,
        assets_gltfmesh: Res<Assets<GltfMesh>>,
    ) -> BlockMeshes {
        let gtlf = assets_gltf.get(&gltfs.gates).unwrap();
        let and = assets_gltfmesh.get(&gtlf.named_meshes["AndGate"]).unwrap();
        let or = assets_gltfmesh.get(&gtlf.named_meshes["OrGate"]).unwrap();
        let xor = assets_gltfmesh.get(&gtlf.named_meshes["XorGate"]).unwrap();
        let not = assets_gltfmesh.get(&gtlf.named_meshes["NotGate"]).unwrap();
        BlockMeshes {
            and: and.primitives[0].mesh.clone().into(),
            or: or.primitives[0].mesh.clone().into(),
            xor: xor.primitives[0].mesh.clone().into(),
            not: not.primitives[0].mesh.clone().into(),
        }
    }
}

pub struct AndGate;

impl BlockType for AndGate {
    fn inputs(&self) -> Vec<Vec2> {
        vec![Vec2::new(-1., -1.), Vec2::new(-1., 1.)]
    }

    fn outputs(&self) -> Vec<Vec2> {
        vec![Vec2::new(1., 0.)]
    }

    fn compute(&self, inputs: Vec<bool>) -> Vec<bool> {
        vec![inputs[0] & inputs[1]]
    }
}

pub trait BlockType: Send + Sync {
    fn inputs(&self) -> Vec<Vec2>;
    fn outputs(&self) -> Vec<Vec2>;
    fn compute(&self, inputs: Vec<bool>) -> Vec<bool>;
    fn tick(&mut self) {}
}

#[derive(Component)]
pub struct Block(pub Box<dyn BlockType>);

impl Block {}

#[derive(Bundle)]
pub struct DisplayBlockBundle {
    pub block: Block,
    pub display: MaterialMesh2dBundle<ColorMaterial>,
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), after_load);
    }
}

fn after_load(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    gltfs: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    commands.insert_resource(BlockMaterials::new(&mut materials));
    commands.insert_resource(BlockMeshes::new(gltfs, assets_gltf, assets_gltfmesh));

    commands.spawn(DisplayBlockBundle {
        block: Block(Box::new(AndGate)),
        display: MaterialMesh2dBundle::default(),
    });
}
