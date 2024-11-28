use super::System;
use crate::{
    components::{Movement, Position},
    tile::{CurrentLevelId, TileMap},
    World,
};

const FADE_SPEED: f32 = 5.0;
const FADE_UPDATE_INTERVAL: f32 = 1.0 / 15.0; // 15 FPS fade

// Resource
pub struct TransitionState {
    state: TransitionPhase,
    fade_alpha: f32,
    time_since_last_fade: f32,
}

#[derive(Clone)]
enum TransitionPhase {
    None,
    FadingOut {
        destination_level_id: String,
        destination_pos: Position,
        position_idx: usize,
    },
    FadingIn,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            state: TransitionPhase::None,
            fade_alpha: 0.0,
            time_since_last_fade: 0.0,
        }
    }
}

pub struct LevelTransitionSystem;

impl LevelTransitionSystem {
    fn apply_fade(pixels: &mut pixels::Pixels, fade_alpha: f32) {
        let frame = pixels.frame_mut();
        let alpha = (fade_alpha * 255.0) as u8;

        for pixel in frame.chunks_exact_mut(4) {
            // Convert everything to u16 to avoid overflow
            let r = pixel[0] as u16;
            let g = pixel[1] as u16;
            let b = pixel[2] as u16;

            // Calculate the blend with white (255) using linear interpolation
            pixel[0] = (r + ((255 - r) * alpha as u16) / 255) as u8;
            pixel[1] = (g + ((255 - g) * alpha as u16) / 255) as u8;
            pixel[2] = (b + ((255 - b) * alpha as u16) / 255) as u8;
            // Alpha channel (pixel[3]) remains unchanged
        }
    }
}

impl System for LevelTransitionSystem {
    fn update(
        &self,
        world: &crate::World,
        pixels: &mut pixels::Pixels,
        _input: &crate::input::Input,
        delta_time: std::time::Duration,
    ) {
        let mut transition_state = world.get_resource_mut::<TransitionState>().unwrap();
        transition_state.time_since_last_fade += delta_time.as_secs_f32();

        let should_update_fade = transition_state.time_since_last_fade >= FADE_UPDATE_INTERVAL;

        match transition_state.state.clone() {
            TransitionPhase::None => {
                if let Some((position_idx, destination_level_id, destination_pos)) =
                    detect_transition(world)
                {
                    transition_state.state = TransitionPhase::FadingOut {
                        destination_level_id,
                        destination_pos,
                        position_idx,
                    };
                }
            }
            TransitionPhase::FadingOut {
                destination_level_id,
                destination_pos,
                position_idx,
            } => {
                if should_update_fade {
                    transition_state.fade_alpha += FADE_SPEED * FADE_UPDATE_INTERVAL;
                    transition_state.time_since_last_fade = 0.0;

                    if transition_state.fade_alpha >= 1.0 {
                        transition_state.fade_alpha = 1.0;

                        change_level(world, destination_level_id);

                        if let Some(mut position_components) =
                            world.borrow_components_mut::<Position>()
                        {
                            if let Some(Some(position)) = position_components.get_mut(position_idx)
                            {
                                *position = destination_pos;
                            }
                        }
                        transition_state.state = TransitionPhase::FadingIn;
                    }
                }
            }
            TransitionPhase::FadingIn => {
                if should_update_fade {
                    transition_state.fade_alpha -= FADE_SPEED * FADE_UPDATE_INTERVAL;
                    transition_state.time_since_last_fade = 0.0;

                    if transition_state.fade_alpha <= 0.0 {
                        transition_state.fade_alpha = 0.0;
                        transition_state.state = TransitionPhase::None;
                    }
                }
            }
        }

        if transition_state.fade_alpha > 0.0 {
            Self::apply_fade(pixels, transition_state.fade_alpha);
        }
    }
}

fn detect_transition(world: &crate::World) -> Option<(usize, String, Position)> {
    let tilemap = world.get_resource::<TileMap>().unwrap();
    let current_level = tilemap.get_level(world);
    let tiles = &current_level.tiles;
    let mut movement_components = world.borrow_components_mut::<Movement>().unwrap();
    let mut position_components = world.borrow_components_mut::<Position>().unwrap();
    let zip = movement_components.iter_mut().zip(position_components.iter_mut());
    let iter =
        zip.filter_map(|(movement, position)| Some((movement.as_mut()?, position.as_mut()?)));

    for (idx, (_, position)) in iter.enumerate() {
        let Some(tile) = position.aligned_tile() else {
            continue;
        };
        if let Some(transition) = &tiles[&(tile.0, tile.1)].transition {
            if let Some(destination) = tilemap.entities.get(&transition.destination) {
                return Some((idx, destination.level_id.clone(), destination.position));
            }
        }
    }
    None
}

fn change_level(world: &World, destination_level_id: String) {
    let mut current_level_id = world.get_resource_mut::<CurrentLevelId>().unwrap();
    current_level_id.0 = destination_level_id;
}
