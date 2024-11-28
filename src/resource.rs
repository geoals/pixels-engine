use crate::{
    camera::Camera,
    spritesheet::CharacterSpritesheet,
    systems::level_transition::ScreenTransition,
    tile::{CurrentLevelId, TileMap},
};

pub struct Resources {
    pub camera: Camera,
    pub character_spritesheet: CharacterSpritesheet,
    // TODO : move this back into tilemap struct
    pub current_level_id: CurrentLevelId,
    pub tilemap: TileMap,
    pub screen_transition: ScreenTransition,
}
