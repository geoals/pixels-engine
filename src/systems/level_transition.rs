use hecs::{With, World};

use super::System;
use crate::{
    components::{Player, Position},
    resource::Resources,
    tile::{CurrentLevelId, TileMap},
    vec2::Vec2,
    TILE_SIZE,
};

const FADE_SPEED: f32 = 5.0;
const FADE_UPDATE_INTERVAL: f32 = 1.0 / 15.0; // 15 FPS fade

// Resource
#[derive(Clone)]
pub struct ScreenTransition {
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
    },
    FadingIn,
}

impl Default for ScreenTransition {
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
        world: &mut World,
        resources: &mut Resources,
        pixels: &mut pixels::Pixels,
        _input: &crate::input::Input,
        delta_time: std::time::Duration,
    ) {
        let transition = &mut resources.screen_transition.clone();
        transition.time_since_last_fade += delta_time.as_secs_f32();

        let should_update_fade = transition.time_since_last_fade >= FADE_UPDATE_INTERVAL;

        match transition.state.clone() {
            TransitionPhase::None => {
                if let Some((destination_level_id, destination_pos)) =
                    detect_transition(world, &resources.tilemap, &resources.current_level_id)
                {
                    transition.state = TransitionPhase::FadingOut {
                        destination_level_id,
                        destination_pos,
                    };
                }
            }
            TransitionPhase::FadingOut {
                destination_level_id,
                destination_pos,
            } => {
                if should_update_fade {
                    transition.fade_alpha += FADE_SPEED * FADE_UPDATE_INTERVAL;
                    transition.time_since_last_fade = 0.0;

                    if transition.fade_alpha >= 1.0 {
                        transition.fade_alpha = 1.0;

                        // TODO: split up actual level change and visual transition stuff
                        // to separate places
                        resources.current_level_id.0 = destination_level_id;
                        for (_, position) in world.query_mut::<With<&mut Position, &Player>>() {
                            *position = destination_pos;
                            let offset = Vec2::new(TILE_SIZE as f32 / 2.0, TILE_SIZE as f32 / 2.0);
                            resources.camera.set_position(*position + offset);
                        }
                        transition.state = TransitionPhase::FadingIn;
                    }
                }
            }
            TransitionPhase::FadingIn => {
                if should_update_fade {
                    transition.fade_alpha -= FADE_SPEED * FADE_UPDATE_INTERVAL;
                    transition.time_since_last_fade = 0.0;

                    if transition.fade_alpha <= 0.0 {
                        transition.fade_alpha = 0.0;
                        transition.state = TransitionPhase::None;
                    }
                }
            }
        }

        if transition.fade_alpha > 0.0 {
            Self::apply_fade(pixels, resources.screen_transition.fade_alpha);
        }

        resources.screen_transition = transition.to_owned();
    }
}

fn detect_transition(
    world: &mut World,
    tilemap: &TileMap,
    current_level_id: &CurrentLevelId,
) -> Option<(String, Position)> {
    let current_level = tilemap.get_level(current_level_id);
    let tiles = &current_level.tiles;

    for (_, position) in world.query_mut::<With<&mut Position, &Player>>() {
        let tile = position.aligned_tile()?;
        if let Some(transition) = &tiles[&(tile.0, tile.1)].transition {
            if let Some(destination) = tilemap.entities.get(&transition.destination) {
                return Some((destination.level_id.clone(), destination.position));
            }
        }
    }

    None
}
