use bevy::prelude::*;

use crate::GameState;

use super::{Block, Input, InputState, Inputs, OutputState, Outputs};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, run_updates.run_if(in_state(GameState::Main)));
    }
}

#[derive(Component, Default)]
pub struct UpdateMarker;

fn run_updates(world: &mut World) {
    let mut update = true;
    while update {
        update = false;
        let mut query =
            world.query_filtered::<(Entity, &Block, &Inputs, &Outputs, &OutputState), With<UpdateMarker>>();
        let updated_entities = query
            .iter(world)
            .filter_map(|(entity, block, inputs, outputs, output_state)| {
                update = true;
                let input_state = inputs
                    .0
                    .iter()
                    .map(|input| match input {
                        Input::Connected(output) => {
                            let e = world.entity(output.entity);
                            if e.contains::<UpdateMarker>() {
                                None
                            } else {
                                Some(e.get::<OutputState>().unwrap().0[output.output])
                            }
                        }
                        Input::Unconnected => Some(false),
                    })
                    .collect::<Option<Vec<_>>>();
                input_state.map(|input_state| {
                    let new_output_state = block.0.compute(&input_state);
                    if new_output_state != output_state.0 {
                        (
                            entity,
                            outputs
                                .0
                                .iter()
                                .flatten()
                                .map(|id| id.entity)
                                .collect::<Vec<_>>(),
                            input_state,
                            new_output_state,
                        )
                    } else {
                        (entity, vec![], input_state, new_output_state)
                    }
                })
            })
            .collect::<Vec<_>>();
        for (entity, next_entities, input_state, output_state) in updated_entities {
            info!("{:?}", input_state);
            world
                .entity_mut(entity)
                .remove::<UpdateMarker>()
                .insert(InputState(input_state))
                .insert(OutputState(output_state));
            for next_entity in next_entities {
                world.entity_mut(next_entity).insert(UpdateMarker);
            }
        }
    }
}
