use crate::{ivec2::IVec2, vec2::Vec2, World};
use ldtk2::Ldtk;
use std::collections::HashMap;

pub struct CurrentLevelId(pub String);

#[derive(Debug)]
pub struct TileMap {
    pub levels: HashMap<String, Level>,
    initial_level_id: String,
    pub tilesize: i64,
    pub player_starting_position: Vec2,
    pub entities: HashMap<EntityId, EntityInstance>,
}

#[derive(Debug)]
pub struct Level {
    pub tiles: HashMap<(i64, i64), TileData>,
    pub tileset_pixels: Vec<u8>,
    pub tileset_width: u32,
    pub tileset_height: u32,
    pub level_id: String,
}

type EntityId = String;

#[derive(Debug)]
pub struct EntityInstance {
    pub position: Vec2,
    pub level_id: String,
}

#[derive(Debug)]
pub struct TileData {
    pub tileset_position: IVec2,
    pub position: IVec2,
    pub traversable: bool,
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub entity_id: EntityId,
    pub destination: EntityId,
}

impl TileMap {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let ldtk = Ldtk::from_path(path)?;

        // Get initial information from first level
        let first_level = &ldtk.levels[0];
        let first_tile_layer = first_level
            .layer_instances
            .as_ref()
            .ok_or("No layers in tile data")?
            .iter()
            .find(|layer| layer.identifier == "Tiles")
            .ok_or("Could not find layer with id Tiles")?;

        let tilesize = ldtk
            .defs
            .tilesets
            .iter()
            .find(|tileset| tileset.rel_path == first_tile_layer.tileset_rel_path)
            .unwrap()
            .tile_grid_size;

        let player_starting_position = Self::get_player_start_position(first_level)?;

        let mut levels = HashMap::new();
        for level_data in &ldtk.levels {
            let level = Self::load_level(level_data, &ldtk.defs.tilesets)?;
            levels.insert(level_data.iid.clone(), level);
        }

        let entities = Self::load_all_entities(&ldtk)?;

        Ok(TileMap {
            levels,
            initial_level_id: ldtk.levels[0].iid.clone(),
            tilesize,
            player_starting_position,
            entities,
        })
    }

    fn get_player_start_position(level: &ldtk2::Level) -> Result<Vec2, Box<dyn std::error::Error>> {
        let entities_layer = level
            .layer_instances
            .as_ref()
            .ok_or("No layers in tile data")?
            .iter()
            .find(|layer| layer.identifier == "Entities")
            .ok_or("Could not find Entities layer")?;

        let player_start = entities_layer
            .entity_instances
            .iter()
            .find(|entity| entity.identifier == "PlayerStart")
            .ok_or("Could not find PlayerStart entity")?;

        Ok(Vec2::new(
            player_start.px[0] as f32,
            player_start.px[1] as f32,
        ))
    }

    fn load_level(
        level_data: &ldtk2::Level,
        tilesets: &[ldtk2::TilesetDefinition],
    ) -> Result<Level, Box<dyn std::error::Error>> {
        let mut tiles = HashMap::new();

        let layer_instances = level_data.layer_instances.as_ref().ok_or("No layers in level")?;

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

        // Collect entrance entities with their grid positions and destination info
        let mut entrance_transitions = HashMap::new();
        for entity in &entities_layer.entity_instances {
            if entity.identifier == "Entrance" {
                if let Some(field) =
                    entity.field_instances.iter().find(|f| f.identifier == "Entity_ref")
                {
                    if let Some(field_value) = &field.value {
                        let destination_entity_id =
                            field_value["entityIid"].as_str().unwrap().to_string();
                        let grid_pos = (entity.grid[0], entity.grid[1]);
                        entrance_transitions.insert(
                            grid_pos,
                            Transition {
                                entity_id: entity.iid.clone(),
                                destination: destination_entity_id,
                            },
                        );
                    }
                }
            }
        }

        // Load tileset for this level
        let tileset = tilesets
            .iter()
            .find(|tileset| tileset.rel_path == tile_layer.tileset_rel_path)
            .unwrap();

        let tileset_path = format!("./assets/{}", tileset.rel_path.as_ref().unwrap());
        let tileset_img = image::open(tileset_path)?;
        let tileset_rgba = tileset_img.to_rgba8();

        // Load tiles
        for tile in &tile_layer.grid_tiles {
            let grid_x = (tile.px[0] / collision_layer.grid_size) as usize;
            let grid_y = (tile.px[1] / collision_layer.grid_size) as usize;
            let grid_index = grid_y * collision_layer.c_wid as usize + grid_x;
            let traversable = collision_layer.int_grid_csv[grid_index] == 0;

            // Check if this tile has an entrance
            let grid_pos = (
                tile.px[0] / tile_layer.grid_size,
                tile.px[1] / tile_layer.grid_size,
            );
            let transition = entrance_transitions.get(&grid_pos).cloned();

            let tile_data = TileData {
                tileset_position: IVec2::new(tile.src[0], tile.src[1]),
                position: IVec2::new(tile.px[0], tile.px[1]),
                traversable,
                transition,
            };

            tiles.insert(
                (
                    tile.px[0] / tile_layer.grid_size,
                    tile.px[1] / tile_layer.grid_size,
                ),
                tile_data,
            );
        }

        Ok(Level {
            tiles,
            tileset_pixels: tileset_rgba.to_vec(),
            tileset_width: tileset_img.width(),
            tileset_height: tileset_img.height(),
            level_id: level_data.iid.clone(),
        })
    }

    fn load_all_entities(
        ldtk: &Ldtk,
    ) -> Result<HashMap<EntityId, EntityInstance>, Box<dyn std::error::Error>> {
        let mut entities = HashMap::new();

        for level in &ldtk.levels {
            let entities_layer = level
                .layer_instances
                .as_ref()
                .ok_or("No layers in level")?
                .iter()
                .find(|layer| layer.identifier == "Entities")
                .ok_or("Could not find Entities layer")?;

            for entity in &entities_layer.entity_instances {
                let entity_instance = EntityInstance {
                    position: Vec2::new(entity.px[0] as f32, entity.px[1] as f32),
                    level_id: level.iid.clone(),
                };

                entities.insert(entity.iid.clone(), entity_instance);
            }
        }

        Ok(entities)
    }

    pub fn get_level(&self, world: &World) -> &Level {
        let current_level_id = &world.get_resource::<CurrentLevelId>().unwrap().0;
        self.levels.get(current_level_id).unwrap()
    }

    pub fn initial_level_id(&self) -> String {
        self.initial_level_id.clone()
    }
}
