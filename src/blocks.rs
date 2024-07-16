mod update;

use std::fmt::Debug;

use bevy::gltf::{Gltf, GltfMesh};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::{HashMap, HashSet};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use update::{UpdateMarker, UpdatePlugin};

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
            red: materials.add(Color::srgb(1., 0., 0.)),
            yellow: materials.add(Color::srgb(1., 1., 0.)),
            green: materials.add(Color::srgb(0., 1., 0.)),
            cyan: materials.add(Color::srgb(0., 1., 1.)),
            blue: materials.add(Color::srgb(0., 0., 1.)),
            magenta: materials.add(Color::srgb(1., 0., 1.)),
            gray: materials.add(Color::srgb(0.5, 0.5, 0.5)),
        }
    }
}

#[derive(Resource)]
pub struct BlockMeshes {
    and: Mesh2dHandle,
    or: Mesh2dHandle,
    xor: Mesh2dHandle,
    not: Mesh2dHandle,

    toggle: Mesh2dHandle,
    led: Mesh2dHandle,
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
        let toggle = assets_gltfmesh.get(&gtlf.named_meshes["Toggle"]).unwrap();
        let led = assets_gltfmesh.get(&gtlf.named_meshes["LED"]).unwrap();
        BlockMeshes {
            and: and.primitives[0].mesh.clone().into(),
            or: or.primitives[0].mesh.clone().into(),
            xor: xor.primitives[0].mesh.clone().into(),
            not: not.primitives[0].mesh.clone().into(),
            toggle: toggle.primitives[0].mesh.clone().into(),
            led: led.primitives[0].mesh.clone().into(),
        }
    }
}

#[derive(Debug)]
pub struct AndGate;

impl BlockType for AndGate {
    fn inputs(&self) -> Vec<Vec2> {
        vec![Vec2::new(-1., -1.), Vec2::new(-1., 1.)]
    }

    fn outputs(&self) -> Vec<Vec2> {
        vec![Vec2::new(1., 0.)]
    }

    fn compute(&self, inputs: &[bool]) -> Vec<bool> {
        vec![inputs[0] & inputs[1]]
    }

    fn mesh(&self, meshes: &BlockMeshes, _inputs: &[bool]) -> Mesh2dHandle {
        meshes.and.clone()
    }

    fn material(&self, materials: &BlockMaterials, _inputs: &[bool]) -> Handle<ColorMaterial> {
        materials.cyan.clone()
    }
}

#[derive(Debug)]
pub struct Toggle(bool);

impl BlockType for Toggle {
    fn inputs(&self) -> Vec<Vec2> {
        vec![]
    }

    fn outputs(&self) -> Vec<Vec2> {
        vec![Vec2::new(0., 0.)]
    }

    fn compute(&self, _inputs: &[bool]) -> Vec<bool> {
        vec![self.0]
    }

    fn mesh(&self, meshes: &BlockMeshes, _inputs: &[bool]) -> Mesh2dHandle {
        meshes.toggle.clone()
    }

    fn material(&self, materials: &BlockMaterials, _inputs: &[bool]) -> Handle<ColorMaterial> {
        if self.0 {
            materials.red.clone()
        } else {
            materials.gray.clone()
        }
    }
}

#[derive(Debug)]
pub struct Led;

impl BlockType for Led {
    fn inputs(&self) -> Vec<Vec2> {
        vec![Vec2::new(0., 0.)]
    }

    fn outputs(&self) -> Vec<Vec2> {
        vec![]
    }

    fn compute(&self, _inputs: &[bool]) -> Vec<bool> {
        vec![]
    }

    fn mesh(&self, meshes: &BlockMeshes, _inputs: &[bool]) -> Mesh2dHandle {
        meshes.led.clone()
    }

    fn material(&self, materials: &BlockMaterials, inputs: &[bool]) -> Handle<ColorMaterial> {
        if inputs[0] {
            materials.red.clone()
        } else {
            materials.gray.clone()
        }
    }
}

pub trait BlockType: Send + Sync + Debug {
    fn inputs(&self) -> Vec<Vec2>;
    fn outputs(&self) -> Vec<Vec2>;
    fn compute(&self, inputs: &[bool]) -> Vec<bool>;

    fn mesh(&self, meshes: &BlockMeshes, inputs: &[bool]) -> Mesh2dHandle;
    fn material(&self, materials: &BlockMaterials, inputs: &[bool]) -> Handle<ColorMaterial>;
}

#[derive(Component, Debug)]
pub struct Block(pub Box<dyn BlockType>);

impl Block {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct OutputId {
    entity: Entity,
    output: usize,
}

impl OutputId {
    pub fn new(entity: Entity, output: usize) -> OutputId {
        OutputId { entity, output }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Input {
    Connected(OutputId),
    Unconnected,
}

impl Input {
    pub fn from_output(entity: Entity, output: usize) -> Input {
        Input::Connected(OutputId::new(entity, output))
    }
}

#[derive(Component)]
pub struct Inputs(pub Vec<Input>);

#[derive(Component)]
struct LastInputs(pub Vec<Input>);

#[derive(Debug, PartialEq, Eq)]
pub struct InputId {
    entity: Entity,
    input: usize,
}

impl InputId {
    pub fn new(entity: Entity, input: usize) -> InputId {
        InputId { entity, input }
    }
}

#[derive(Component, Debug, PartialEq, Eq)]
struct Outputs(pub Vec<Vec<InputId>>);

#[derive(Component)]
struct OutputState(pub Vec<bool>);

#[derive(Component)]
struct InputState(pub Vec<bool>);

#[derive(Bundle)]
pub struct DisplayBlockBundle {
    pub block: Block,
    pub inputs: Inputs,
    pub display: ColorMesh2dBundle,
    pub update_marker: UpdateMarker,
}

impl Default for DisplayBlockBundle {
    fn default() -> Self {
        DisplayBlockBundle {
            block: Block(Box::new(Toggle(false))),
            inputs: Inputs(vec![]),
            display: Default::default(),
            update_marker: Default::default(),
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UpdatePlugin)
            .add_systems(Startup, init)
            .add_systems(OnExit(GameState::Loading), after_load)
            .add_systems(
                Update,
                (init_blocks, update_outputs, update_display)
                    .chain()
                    .run_if(in_state(GameState::Main)),
            );
    }
}

fn init(mut commands: Commands, mut virtual_time: ResMut<Time<Virtual>>) {
    commands.insert_resource(Time::<Fixed>::from_hz(0.5));
    virtual_time.pause();
}

fn after_load(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    gltfs: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    commands.insert_resource(BlockMaterials::new(&mut materials));
    let meshes = BlockMeshes::new(gltfs, assets_gltf, assets_gltfmesh);

    commands.spawn(ColorMesh2dBundle {
        mesh: meshes.and.clone(),
        material: materials.add(ColorMaterial::from_color(Color::srgb(1., 1., 1.))),
        transform: Transform::from_xyz(2., 0., 0.),
        ..Default::default()
    });

    commands.insert_resource(meshes);

    let toggle0 = commands
        .spawn(DisplayBlockBundle {
            block: Block(Box::new(Toggle(true))),
            inputs: Inputs(vec![]),
            display: MaterialMesh2dBundle {
                transform: Transform::from_xyz(-3., -1., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    let toggle1 = commands
        .spawn(DisplayBlockBundle {
            block: Block(Box::new(Toggle(true))),
            inputs: Inputs(vec![]),
            display: MaterialMesh2dBundle {
                transform: Transform::from_xyz(-3., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let and = commands
        .spawn(DisplayBlockBundle {
            block: Block(Box::new(AndGate)),
            inputs: Inputs(vec![
                Input::from_output(toggle0, 0),
                Input::from_output(toggle1, 0),
            ]),
            display: MaterialMesh2dBundle::default(),
            ..Default::default()
        })
        .id();

    commands.spawn(DisplayBlockBundle {
        block: Block(Box::new(Led)),
        inputs: Inputs(vec![Input::from_output(and, 0)]),
        display: MaterialMesh2dBundle {
            transform: Transform::from_xyz(3., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    virtual_time.unpause();
}

type InitBlocksFilter = Or<(Added<Inputs>, Without<Outputs>)>;

/// Initializes newly created blocks by adding [`Outputs`] and [`LastInputs`]
fn init_blocks(mut commands: Commands, query: Query<(Entity, &Block, &Inputs), InitBlocksFilter>) {
    for (entity, block, inputs) in &query {
        commands
            .entity(entity)
            .insert(Outputs(
                block.0.outputs().into_iter().map(|_| vec![]).collect(),
            ))
            .insert(OutputState(
                block.0.outputs().into_iter().map(|_| false).collect(),
            ))
            .insert(InputState(
                block.0.inputs().into_iter().map(|_| false).collect(),
            ))
            .insert(LastInputs(inputs.0.clone()));
    }
}

type ChangedInputsFilter = Or<(Changed<Inputs>, Added<Inputs>)>;

/// Updates [`Outputs`] of blocks to match the outputs of blocks.
///
/// ## Algorithm
///
/// - Get all of the changed and new inputs, and generate a set of all affected outputs.
/// - Get all inputs the affected outputs are connected to.
/// - Update the affected outputs.
fn update_outputs(
    mut changed_inputs: Query<(&Inputs, &mut LastInputs), ChangedInputsFilter>,
    query_inputs: Query<(Entity, &Inputs)>,
    mut query_outputs: Query<(Entity, &mut Outputs)>,
) {
    let mut outputs_to_update: HashSet<OutputId> = HashSet::new();
    for (inputs, mut last_inputs) in &mut changed_inputs {
        for (input, last_input) in inputs.0.iter().zip(&last_inputs.0) {
            if input != last_input {
                if let Input::Connected(output) = input {
                    outputs_to_update.insert(output.clone());
                }
                if let Input::Connected(output) = last_input {
                    outputs_to_update.insert(output.clone());
                }
            }
        }
        *last_inputs = LastInputs(inputs.0.clone());
    }

    let mut outputs_to_update: HashMap<OutputId, Vec<InputId>> =
        outputs_to_update.into_iter().map(|o| (o, vec![])).collect();
    for (entity, inputs) in &query_inputs {
        for (input_id, input) in inputs.0.iter().enumerate() {
            if let Input::Connected(output) = input {
                if let Some(o) = outputs_to_update.get_mut(output) {
                    o.push(InputId::new(entity, input_id))
                }
            }
        }
    }

    for (entity, mut outputs) in &mut query_outputs {
        if outputs_to_update
            .keys()
            .any(|output| output.entity == entity)
        {
            for (output_id, output) in outputs.0.iter_mut().enumerate() {
                if let Some(inputs) = outputs_to_update.remove(&OutputId::new(entity, output_id)) {
                    *output = inputs
                }
            }
        }
    }
}

type UpdateDisplayFilter = Or<(Changed<Block>, Changed<InputState>, Added<Mesh2dHandle>)>;

fn update_display(
    mut query: Query<
        (
            &Block,
            &mut Mesh2dHandle,
            &mut Handle<ColorMaterial>,
            &InputState,
        ),
        UpdateDisplayFilter,
    >,
    meshes: Res<BlockMeshes>,
    materials: Res<BlockMaterials>,
) {
    for (block, mut mesh, mut material, input_state) in &mut query {
        *mesh = block.0.mesh(&meshes, &input_state.0);
        *material = block.0.material(&materials, &input_state.0);
    }
}
