use std::collections::HashMap;

use ldtk2::Ldtk;

#[derive(Default, Debug)]
pub struct TileMap {
    pub tiles: HashMap<(i64, i64), TileData>,
    pub tileset_pixels: Vec<u8>,
    pub tileset_width: u32,
    pub tileset_height: u32,
    pub tilesize: u32,
}

#[derive(Debug)]
pub struct TileData {
    pub tile_id: i32,
    pub world_x: i64,
    pub world_y: i64,
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

        for tile in &tile_layer.grid_tiles {
            // not traversable if tile.px[0], tile.px[1] is in collision layer
            // collision layer is collision_layer.int_grid_csv containing numbers, and 1 or 2 means
            // not traversable

            let traversable = collision_layer.int_grid_csv[(tile.px[1] / collision_layer.grid_size)
                as usize
                * collision_layer.c_wid as usize
                + (tile.px[0] / collision_layer.grid_size) as usize]
                == 0;

            let tile_data = TileData {
                tile_id: tile.t as i32,
                world_x: tile.px[0],
                world_y: tile.px[1],
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
            tilesize: tileset.tile_grid_size as u32,
        })
    }
}
