use crate::{vec2::Vec2, TILE_SIZE};

#[derive(Debug)]
pub struct Camera {
    // Position of camera in world coordinates
    position: Vec2,
    // Size of the viewport in pixels
    viewport_width: u32,
    viewport_height: u32,
}

impl Camera {
    pub fn new(viewport_width: u32, viewport_height: u32) -> Self {
        Self {
            position: Vec2::ZERO,
            viewport_width,
            viewport_height,
        }
    }

    /// Get camera position
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Set camera position
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let screen_center = Vec2::new(
            self.viewport_width as f32 / 2.0,
            self.viewport_height as f32 / 2.0,
        );
        world_pos - self.position + screen_center
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let screen_center = Vec2::new(
            self.viewport_width as f32 / 2.0,
            self.viewport_height as f32 / 2.0,
        );
        let offset = screen_pos - screen_center;
        self.position + offset
    }

    /// Check if a world position is visible on screen
    pub fn is_visible(&self, world_pos: Vec2) -> bool {
        let screen_pos = self.world_to_screen(world_pos);
        screen_pos.x >= -(TILE_SIZE as f32)
            && screen_pos.x <= self.viewport_width as f32 + TILE_SIZE as f32
            && screen_pos.y >= -(TILE_SIZE as f32)
            && screen_pos.y <= self.viewport_height as f32 + TILE_SIZE as f32
    }
}
