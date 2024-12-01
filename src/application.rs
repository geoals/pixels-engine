use pixels_engine::components::FireSpell;
use pixels_engine::components::Light;
use pixels_engine::components::Player;
use pixels_engine::resource::LightMap;
use pixels_engine::spritesheet::EffectsSpritesheet;
use pixels_engine::systems::cast_spell::CastSpellSystem;
use pixels_engine::systems::light_control::LightControlSystem;
use pixels_engine::systems::light_render::LightRenderSystem;
use pixels_engine::systems::light_render::LightUpdateSystem;
use pixels_engine::systems::spell_effect::SpellEffectRenderSystem;
use pixels_engine::vec2::Vec2;
use pixels_engine::SCALE_FACTOR;

use hecs::World;
use pixels::Pixels;
use pixels::SurfaceTexture;
use pixels_engine::camera::Camera;
use pixels_engine::components::AnimatedSprite;
use pixels_engine::components::Movement;
use pixels_engine::components::Position;
use pixels_engine::components::SpriteType;
use pixels_engine::input::Input;
use pixels_engine::resource::Resources;
use pixels_engine::spritesheet::CharacterSpritesheet;
use pixels_engine::spritesheet::Spritesheet;
use pixels_engine::systems::camera::CameraFollowSystem;
use pixels_engine::systems::character_animation::CharacterAnimationSystem;
use pixels_engine::systems::debug_grid::DebugGridSystem;
use pixels_engine::systems::level_transition::LevelTransitionSystem;
use pixels_engine::systems::level_transition::ScreenTransition;
use pixels_engine::systems::movement::MovementSystem;
use pixels_engine::systems::sprite_render::SpriteRenderSystem;
use pixels_engine::systems::tile_animation::TileAnimationSystem;
use pixels_engine::systems::tile_render::TileRenderSystem;
use pixels_engine::systems::SystemContainer;
use pixels_engine::tile::CurrentLevelId;
use pixels_engine::tile::TileMap;
use pixels_engine::SCREEN_HEIGHT;
use pixels_engine::SCREEN_WIDTH;
use pixels_engine::TILE_SIZE;
use std::time::Duration;
use winit::window::Window;

pub struct Application {
    systems: SystemContainer,
    world: World,
    resources: Resources,
    input: Input,
    pixels: Pixels,
    pub delta_time: Duration,
}

impl Application {
    pub fn new(window: &Window) -> Self {
        let tilemap = TileMap::load("./assets/world.ldtk").unwrap();
        let player_pos = tilemap.player_starting_position;

        let camera = Camera::new(
            player_pos + Vec2::new(TILE_SIZE as f32 / 2.0, TILE_SIZE as f32 / 2.0),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        let mut world = hecs::World::new();
        world.spawn((
            AnimatedSprite::new(SpriteType::Player),
            Position::new(player_pos.x, player_pos.y),
            Movement::new(48.0),
            Player,
            Light::new(115.0, 1.0, [1.0, 1.0, 1.0]),
            FireSpell,
        ));

        Self {
            systems: Self::set_up_systems(),
            input: Input::new(),
            pixels: Self::set_up_pixels_frame_buffer(window),
            delta_time: Duration::ZERO,
            world,
            resources: Resources {
                camera,
                character_spritesheet: CharacterSpritesheet(
                    Spritesheet::new("./assets/characters_spritesheet.png", 16, 16).unwrap(),
                ),
                effects_spritesheet: EffectsSpritesheet(
                    Spritesheet::new("./assets/effects.png", 16, 16).unwrap(),
                ),
                current_level_id: CurrentLevelId(tilemap.initial_level_id()),
                tilemap,
                screen_transition: Default::default(),
                light_map: Default::default(),
            },
        }
    }

    fn set_up_systems() -> SystemContainer {
        let mut systems = SystemContainer::new();

        systems.add_update_system(MovementSystem);
        systems.add_update_system(CharacterAnimationSystem);
        systems.add_update_system(TileAnimationSystem);
        systems.add_update_system(CameraFollowSystem);
        systems.add_update_system(LightUpdateSystem);
        systems.add_update_system(CastSpellSystem);

        systems.add_render_system(TileRenderSystem);
        systems.add_render_system(SpriteRenderSystem);
        systems.add_render_system(SpellEffectRenderSystem);
        // systems.add_render_system(LightRenderSystem);
        systems.add_render_system(LevelTransitionSystem);

        if cfg!(feature = "debug") {
            systems.add_update_system(DebugGridSystem);
            systems.add_update_system(LightControlSystem);
        }

        systems
    }

    fn set_up_pixels_frame_buffer(window: &Window) -> Pixels {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(
            size.width / SCALE_FACTOR,
            size.height / SCALE_FACTOR,
            &window,
        );
        let mut pixels = Pixels::new(
            size.width / SCALE_FACTOR,
            size.height / SCALE_FACTOR,
            surface_texture,
        )
        .unwrap();
        pixels.enable_vsync(false);
        pixels
    }

    pub fn update(&mut self) {
        if self.systems.should_update(self.delta_time) {
            let fixed_delta_time = self.systems.get_fixed_delta_time();
            for system in self.systems.get_update_systems() {
                system.update(
                    &mut self.world,
                    &mut self.resources,
                    &mut self.pixels,
                    &self.input,
                    fixed_delta_time,
                );
            }
        }

        for system in self.systems.get_render_systems() {
            system.update(
                &mut self.world,
                &mut self.resources,
                &mut self.pixels,
                &self.input,
                self.delta_time,
            );
        }
    }

    pub fn draw(&mut self) {
        self.pixels.render().expect("Should draw the pixel buffer to screen");
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        for byte in self.pixels.frame_mut().iter_mut() {
            *byte = 0;
        }
    }

    pub fn process_input_events(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.input.process_events(event)
    }
}
