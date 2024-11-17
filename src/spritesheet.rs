use std::path::Path;

use image::{DynamicImage, GenericImageView};

pub struct CharacterSpritesheet(pub Spritesheet);

pub struct Spritesheet {
    image: DynamicImage,
    sprite_width: u32,
    sprite_height: u32,
}

impl Spritesheet {
    pub fn new<P: AsRef<Path>>(
        path: P,
        sprite_width: u32,
        sprite_height: u32,
    ) -> Result<Self, image::ImageError> {
        let image = image::open(path)?;
        Ok(Self {
            image,
            sprite_width,
            sprite_height,
        })
    }

    /// Gets the pixel data for a sprite at the given position in the spritesheet
    pub fn get_sprite(&self, sprite_x: u32, sprite_y: u32) -> Option<Vec<u8>> {
        // Check if the sprite is within bounds
        if sprite_x + self.sprite_width > self.image.width()
            || sprite_y + self.sprite_height > self.image.height()
        {
            return None;
        }

        // Extract sprite pixels
        let mut sprite_data =
            Vec::with_capacity((self.sprite_width * self.sprite_height * 4) as usize);

        for y in 0..self.sprite_height {
            for x in 0..self.sprite_width {
                let pixel = self.image.get_pixel(sprite_x + x, sprite_y + y);
                sprite_data.extend_from_slice(&pixel.0);
            }
        }

        Some(sprite_data)
    }

    /// Draw a sprite directly to a pixel buffer at the specified position
    #[allow(clippy::too_many_arguments)]
    pub fn draw_sprite_to_buffer(
        &self,
        sprite_x: u32, // X position in spritesheet
        sprite_y: u32, // Y position in spritesheet
        target: &mut [u8],
        target_width: u32,
        target_height: u32,
        dest_x: u32, // X position in target
        dest_y: u32, // Y position in target
    ) -> Option<()> {
        // TODO: don't get the sprite data every time
        let sprite_data = self.get_sprite(sprite_x, sprite_y)?;
        let scale = 4;

        for y in 0..self.sprite_height * scale {
            let target_y = dest_y + y;
            if target_y >= target_height {
                continue;
            }

            // Calculate which row of the original sprite we're on
            let sprite_y = y / scale;

            for x in 0..self.sprite_width * scale {
                let target_x = dest_x + x;
                if target_x >= target_width {
                    continue;
                }

                // Calculate which column of the original sprite we're on
                let sprite_x = x / scale;

                let sprite_idx = ((sprite_y * self.sprite_width + sprite_x) * 4) as usize;
                let target_idx = ((target_y * target_width + target_x) * 4) as usize;

                // Only draw non-transparent pixels
                if sprite_data[sprite_idx + 3] > 0 {
                    target[target_idx..target_idx + 4]
                        .copy_from_slice(&sprite_data[sprite_idx..sprite_idx + 4]);
                }
            }
        }
        Some(())
    }
}
