use std::collections::HashMap;

use ldtk2::Ldtk;

use crate::{ivec2::IVec2, vec2::Vec2};

#[derive(Debug)]
pub struct TileMap {
    pub levels: HashMap<String, Level>,
    current_level_id: String,
    pub tileset_pixels: Vec<u8>,
    pub tileset_width: u32,
    pub tileset_height: u32,
    pub tilesize: i64,
}

#[derive(Debug)]
pub struct Level {
    pub tiles: HashMap<(i64, i64), TileData>,
    pub player_starting_position: Vec2,
    // pub entrances: HashMap<String, Entrance>,
}

#[derive(Debug)]
pub struct Entrance {
    pub position: Vec2,
    pub destination: LevelDestination,
}

#[derive(Debug)]
pub struct LevelDestination {
    pub level_iid: String,
    pub entrance_iid: String,
}

#[derive(Debug)]
pub struct TileData {
    /// Pixel coordinate of the tile in the tileset
    pub tileset_position: IVec2,
    /// Pixel coordinate of the tile in the world
    pub position: IVec2,
    pub traversable: bool,
}

// struct Door {
//     destination:
// }

impl TileMap {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let ldtk = Ldtk::from_path(path)?;
        let mut levels = HashMap::new();

        let level = &ldtk.levels[0];
        let tile_layer = level
            .layer_instances
            .as_ref()
            .ok_or("No layers in tile data")?
            .iter()
            .find(|layer| layer.identifier == "Tiles")
            .ok_or("Could not find layer with id Tiles")?;

        // Load tileset image
        let tileset = &ldtk
            .defs
            .tilesets
            .iter()
            .find(|tileset| tileset.rel_path == tile_layer.tileset_rel_path)
            .unwrap();

        let path = format!("./assets/{}", tileset.rel_path.as_ref().unwrap());

        let tileset_img = image::open(path)?;
        let tileset_rgba = tileset_img.to_rgba8();

        for level_data in &ldtk.levels {
            let level = Self::load_level(level_data)?;
            levels.insert(level_data.iid.clone(), level);
        }

        Ok(TileMap {
            levels,
            current_level_id: ldtk.levels[0].iid.clone(),
            tileset_pixels: tileset_rgba.to_vec(),
            tileset_width: tileset_img.width(),
            tileset_height: tileset_img.height(),
            tilesize: tileset.tile_grid_size,
        })
    }

    fn load_level(level_data: &ldtk2::Level) -> Result<Level, Box<dyn std::error::Error>> {
        let mut tiles = HashMap::new();

        let layer_instances = level_data
            .layer_instances
            .as_ref()
            .ok_or("No layers in level")?;

        let tile_layer = layer_instances
            .iter()
            .find(|layer| layer.identifier == "Tiles")
            .ok_or("Could not find Tiles layer")?;

        let collision_layer = layer_instances
            .iter()
            .find(|layer| layer.identifier == "Collision")
            .ok_or("Could not find Collision layer")?;

        let entities_layer = layer_instances
            .iter()
            .find(|layer| layer.identifier == "Entities")
            .ok_or("Could not find Entities layer")?;

        // Load tiles
        for tile in &tile_layer.grid_tiles {
            let grid_x = (tile.px[0] / collision_layer.grid_size) as usize;
            let grid_y = (tile.px[1] / collision_layer.grid_size) as usize;
            let grid_index = grid_y * collision_layer.c_wid as usize + grid_x;
            let traversable = collision_layer.int_grid_csv[grid_index] == 0;

            let tile_data = TileData {
                tileset_position: IVec2::new(tile.src[0], tile.src[1]),
                position: IVec2::new(tile.px[0], tile.px[1]),
                traversable,
            };

            tiles.insert(
                (
                    tile.px[0] / tile_layer.grid_size,
                    tile.px[1] / tile_layer.grid_size,
                ),
                tile_data,
            );
        }

        // Get player starting position
        let player_start = entities_layer
            .entity_instances
            .iter()
            .find(|entity| entity.identifier == "PlayerStart")
            .ok_or("Could not find PlayerStart entity")?;

        Ok(Level {
            tiles,
            player_starting_position: Vec2::new(
                player_start.px[0] as f32,
                player_start.px[1] as f32,
            ),
        })
    }

    pub fn current_level(&self) -> &Level {
        self.levels.get(&self.current_level_id).unwrap()
    }
}
