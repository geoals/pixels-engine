use crate::{
    camera::Camera,
    spritesheet::{CharacterSpritesheet, EffectsSpritesheet},
    systems::level_transition::ScreenTransition,
    tile::{CurrentLevelId, TileMap},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub struct Resources {
    pub camera: Camera,
    pub character_spritesheet: CharacterSpritesheet,
    pub effects_spritesheet: EffectsSpritesheet,
    // TODO : move this back into tilemap struct
    pub current_level_id: CurrentLevelId,
    pub tilemap: TileMap,
    pub screen_transition: ScreenTransition,
    pub light_map: LightMap,
}

pub struct LightMap {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
}

impl Default for LightMap {
    fn default() -> Self {
        let scale = 1;
        let width = SCREEN_WIDTH / scale;
        let height = SCREEN_HEIGHT / scale;

        Self {
            buffer: vec![0; (width * height * 4) as usize],
            width,
            height,
            scale,
        }
    }
}

impl LightMap {
    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
