use std::collections::HashMap;

use ldtk2::Ldtk;

use crate::{ivec2::IVec2, vec2::Vec2};

#[derive(Debug)]
pub struct TileMap {
    pub tiles: HashMap<(i64, i64), TileData>,
    pub tileset_pixels: Vec<u8>,
    pub tileset_width: u32,
    pub tileset_height: u32,
    pub tilesize: i64,
    pub player_starting_position: Vec2,
}

#[derive(Debug)]
pub struct TileData {
    /// Pixel coordinate of the tile in the tileset
    pub tileset_position: IVec2,
    /// Pixel coordinate of the tile in the world
    pub position: IVec2,
    pub tile_id: i64,
    pub traversable: bool,
}

impl TileMap {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let ldtk = Ldtk::from_path(path)?;
        let mut tiles = HashMap::new();

        let level = &ldtk.levels[0];

        let tile_layer = level
            .layer_instances
            .as_ref()
            .ok_or("No layers in tile data")?
            .iter()
            .find(|layer| layer.identifier == "Tiles")
            .ok_or("Could not find layer with id Tiles")?;

        let collision_layer = level
            .layer_instances
            .as_ref()
            .ok_or("No layers in tile data")?
            .iter()
            .find(|layer| layer.identifier == "Collision")
            .ok_or("Could not find layer with id Collision")?;

        let entities_layer = level
            .layer_instances
            .as_ref()
            .ok_or("No layers in tile data")?
            .iter()
            .find(|layer| layer.identifier == "Entities")
            .ok_or("Could not find layer with id Entities")?;

        let player_starting_position = entities_layer
            .entity_instances
            .iter()
            .find(|entity| entity.identifier == "PlayerStart")
            .ok_or("Could not find PlayerStart entity")?;

        for tile in &tile_layer.grid_tiles {
            let grid_x = (tile.px[0] / collision_layer.grid_size) as usize;
            let grid_y = (tile.px[1] / collision_layer.grid_size) as usize;
            let grid_index = grid_y * collision_layer.c_wid as usize + grid_x;
            let traversable = collision_layer.int_grid_csv[grid_index] == 0;

            let tile_data = TileData {
                tileset_position: IVec2::new(tile.src[0], tile.src[1]),
                position: IVec2::new(tile.px[0], tile.px[1]),
                tile_id: tile.t,
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

        Ok(TileMap {
            tiles,
            tileset_pixels: tileset_rgba.to_vec(),
            tileset_width: tileset_img.width(),
            tileset_height: tileset_img.height(),
            tilesize: tileset.tile_grid_size,
            player_starting_position: Vec2::new(
                player_starting_position.px[0] as f32,
                player_starting_position.px[1] as f32,
            ),
        })
    }
}
