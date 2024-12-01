use hecs::{With, World};

use crate::{
    components::{FireSpell, Movement, Position, SpellEffect, SpellEffectType},
    movement_util::Direction,
    vec2::Vec2,
    TILE_SIZE,
};

use super::System;

pub struct CastSpellSystem;

impl System for CastSpellSystem {
    fn update(
        &self,
        world: &mut hecs::World,
        _: &mut crate::resource::Resources,
        _: &mut pixels::Pixels,
        input: &crate::input::Input,
        _: std::time::Duration,
    ) {
        let mut spells_to_cast = Vec::new();
        for (_, (position, movement)) in
            world.query::<With<(&Position, &Movement), &FireSpell>>().iter()
        {
            if input.space() {
                spells_to_cast.push((*position, movement.direction));
            }
        }

        for (caster_position, caster_direction) in spells_to_cast {
            cast_spell(
                world,
                caster_position,
                caster_direction,
                SpellEffectType::Fireball,
            );
        }
    }
}

pub fn cast_spell(
    world: &mut World,
    caster_position: Position,
    caster_direction: Direction,
    spell_type: SpellEffectType,
) {
    for i in 1..=5 {
        let offset =
            caster_direction.to_vector() * (TILE_SIZE as f32 * i as f32) + Vec2::new(0.0, -4.0);
        let effect_position = caster_position + offset;

        world.spawn((
            SpellEffect::new(spell_type),
            Position::new(effect_position.x, effect_position.y),
        ));
    }
}
