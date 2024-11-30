use pixels_engine::components::Light;
use pixels_engine::components::Player;
use pixels_engine::resource::LightMap;
use pixels_engine::systems::light_render::LightRenderSystem;
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
        let player_starting_position = tilemap.player_starting_position;

        let mut world = hecs::World::new();
        world.spawn((
            AnimatedSprite::new(SpriteType::Player),
            Position::new(player_starting_position.x, player_starting_position.y),
            Movement::new(48.0),
            Player,
            Light::new(
                Vec2::new(player_starting_position.x, player_starting_position.y),
                80.0, // larger radius for player light
                1.0,
                [0.8, 0.8, 1.0], // slightly blue tint
            ),
        ));
        world.spawn((Light::new(
            Vec2::new(
                player_starting_position.x - 5.0 * TILE_SIZE as f32,
                player_starting_position.y,
            ),
            40.0,
            1.0,
            [0.8, 0.8, 1.0], // slightly blue tint
        ),));

        Self {
            systems: Self::set_up_systems(),
            input: Input::new(),
            pixels: Self::set_up_pixels_frame_buffer(window),
            delta_time: Duration::ZERO,
            world,
            resources: Resources {
                camera: Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                character_spritesheet: CharacterSpritesheet(
                    Spritesheet::new("./assets/characters_spritesheet.png", 16, 16).unwrap(),
                ),
                current_level_id: CurrentLevelId(tilemap.initial_level_id()),
                tilemap,
                screen_transition: ScreenTransition::default(),
                light_map: LightMap::default(),
            },
        }
    }

    fn set_up_systems() -> SystemContainer {
        let mut systems = SystemContainer::new();

        systems.add(MovementSystem);
        systems.add(CharacterAnimationSystem);
        systems.add(TileAnimationSystem);
        systems.add(CameraFollowSystem);
        if cfg!(feature = "debug") {
            systems.add(DebugGridSystem);
        }
        systems.add(TileRenderSystem);
        systems.add(SpriteRenderSystem);
        systems.add(LevelTransitionSystem);
        systems.add(LightRenderSystem);

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
        for system in self.systems.all() {
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
        self.pixels.render().unwrap();
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
