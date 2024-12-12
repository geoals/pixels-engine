use crate::{
    camera::Camera,
    spritesheet::{Spritesheet, SpritesheetConfig},
    systems::level_transition::ScreenTransition,
    tile::TileMap,
    vec2::Vec2,
    SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

pub struct Resources {
    pub camera: Camera,
    pub character_spritesheet: CharacterSpritesheet,
    pub effects_spritesheet: EffectsSpritesheet,
    pub tilemap: TileMap,
    pub screen_transition: ScreenTransition,
    pub light_map: LightMap,
}

impl Resources {
    pub fn new(tilemap: TileMap, player_pos: Vec2) -> Self {
        let camera = Camera::new(
            player_pos + Vec2::new(TILE_SIZE as f32 / 2.0, TILE_SIZE as f32 / 2.0),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        Self {
            camera,
            tilemap,
            character_spritesheet: Default::default(),
            effects_spritesheet: Default::default(),
            screen_transition: Default::default(),
            light_map: Default::default(),
        }
    }
}

pub struct CharacterSpritesheet(pub Spritesheet);

impl Default for CharacterSpritesheet {
    fn default() -> Self {
        CharacterSpritesheet(
            Spritesheet::new(
                "./assets/char.png",
                SpritesheetConfig {
                    padding: 1,
                    ..Default::default()
                },
            )
            .unwrap(),
        )
    }
}

pub struct EffectsSpritesheet(pub Spritesheet);

impl Default for EffectsSpritesheet {
    fn default() -> Self {
        EffectsSpritesheet(Spritesheet::new("./assets/effects.png", Default::default()).unwrap())
    }
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
