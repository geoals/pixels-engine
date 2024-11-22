use std::{collections::HashMap, path::Path};

use image::{DynamicImage, GenericImageView};

pub struct CharacterSpritesheet(pub Spritesheet);

pub struct Spritesheet {
    image: DynamicImage,
    sprite_width: u32,
    sprite_height: u32,
    sprite_cache: HashMap<(u32, u32), Vec<u8>>,
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
            sprite_cache: HashMap::new(),
        })
    }

    /// Gets the pixel data for a sprite at the given position in the spritesheet
    pub fn get_sprite(&mut self, sprite_x: u32, sprite_y: u32) -> Option<&Vec<u8>> {
        // Check if the sprite is within bounds
        if sprite_x + self.sprite_width > self.image.width()
            || sprite_y + self.sprite_height > self.image.height()
        {
            return None;
        }

        if self.sprite_cache.contains_key(&(sprite_x, sprite_y)) {
            return self.sprite_cache.get(&(sprite_x, sprite_y));
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

        self.sprite_cache.insert((sprite_x, sprite_y), sprite_data);
        self.sprite_cache.get(&(sprite_x, sprite_y))
    }

    /// Draw a sprite directly to a pixel buffer at the specified position
    #[allow(clippy::too_many_arguments)]
    pub fn draw_sprite_to_buffer(
        &mut self,
        sprite_x: u32,
        sprite_y: u32,
        target: &mut [u8],
        target_width: u32,
        target_height: u32,
        dest_x: i32,
        dest_y: i32,
    ) {
        let sprite_height = self.sprite_height;
        let sprite_width = self.sprite_width;
        let sprite_data = self.get_sprite(sprite_x, sprite_y).unwrap();

        for y in 0..sprite_height {
            let target_y = dest_y + y as i32;
            if target_y >= target_height as i32 || target_y < 0 {
                continue;
            }

            for x in 0..sprite_width {
                let target_x = dest_x + x as i32;
                if target_x >= target_width as i32 || target_x < 0 {
                    continue;
                }

                let sprite_idx = ((y * sprite_width + x) * 4) as usize;
                if sprite_data[sprite_idx + 3] > 0 {
                    let target_idx = ((target_y * target_width as i32 + target_x) * 4) as usize;
                    target[target_idx..target_idx + 4]
                        .copy_from_slice(&sprite_data[sprite_idx..sprite_idx + 4]);
                }
            }
        }
    }
}
