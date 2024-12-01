use super::System;
use crate::{
    components::{Light, Player},
    input::Input,
    resource::Resources,
};
use hecs::With;
use pixels::Pixels;
use std::time::Duration;

pub struct LightControlSystem;

impl System for LightControlSystem {
    fn update(
        &self,
        world: &mut hecs::World,
        _resources: &mut Resources,
        _pixels: &mut Pixels,
        input: &Input,
        delta_time: Duration,
    ) {
        for (_, light) in world.query_mut::<With<&mut Light, &Player>>() {
            const CHANGE_RATE: f32 = 0.5;

            match (input.shift(), input.k(), input.j()) {
                (true, true, _) => {
                    light.radius =
                        (light.radius + CHANGE_RATE * 50.0 * delta_time.as_secs_f32()).min(200.0);
                }
                (true, _, true) => {
                    light.radius =
                        (light.radius - CHANGE_RATE * 50.0 * delta_time.as_secs_f32()).max(0.0);
                }
                (false, true, _) => {
                    light.intensity =
                        (light.intensity + CHANGE_RATE * delta_time.as_secs_f32()).min(2.0);
                }
                (false, _, true) => {
                    light.intensity =
                        (light.intensity - CHANGE_RATE * delta_time.as_secs_f32()).max(0.0);
                }
                _ => {}
            }
        }
    }
}
