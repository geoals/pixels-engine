use crate::{ivec2::IVec2, movement_util::Direction, vec2::Vec2, TILE_SIZE};
use ldtk2::Ldtk;
use std::{collections::HashMap, time::Duration};

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
    pub indoors: bool,
}

type EntityId = String;

#[derive(Debug)]
pub struct EntityInstance {
    pub position: Vec2,
    pub level_id: String,
    pub direction: Option<Direction>,
}

#[derive(Debug)]
pub struct TileData {
    pub tileset_position: IVec2,
    pub position: IVec2,
    pub traversable: bool,
    pub transition: Option<Transition>,
    pub animation: Option<TileAnimation>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub entity_id: EntityId,
    pub destination: EntityId,
}

#[derive(Debug, Clone)]
pub struct TileAnimation {
    pub frames: Vec<IVec2>,         // Positions of each frame in the tileset
    pub frame_duration: Duration,   // How long each frame should display
    pub current_frame: usize,       // Current frame index
    pub accumulated_time: Duration, // Time accumulated since last frame change
}

impl TileAnimation {
    pub fn new(frames: Vec<IVec2>, frame_duration: Duration) -> Self {
        Self {
            frames,
            frame_duration,
            current_frame: 0,
            accumulated_time: Duration::ZERO,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.accumulated_time += delta_time;
        if self.accumulated_time >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.accumulated_time -= self.frame_duration;
        }
    }

    pub fn current_position(&self) -> IVec2 {
        self.frames[self.current_frame]
    }
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

        let player_start = Self::get_player_start(&ldtk.levels)?;

        let mut levels = HashMap::new();
        for level_data in &ldtk.levels {
            let level = Self::load_level(level_data, &ldtk.defs.tilesets)?;
            levels.insert(level_data.iid.clone(), level);
        }

        let entities = Self::load_all_entities(&ldtk)?;

        Ok(TileMap {
            levels,
            initial_level_id: player_start.1,
            tilesize,
            player_starting_position: player_start.0,
            entities,
        })
    }

    fn get_player_start(
        levels: &[ldtk2::Level],
    ) -> Result<(Vec2, String), Box<dyn std::error::Error>> {
        for level in levels {
            if let Some(layers) = &level.layer_instances {
                if let Some(entities_layer) =
                    layers.iter().find(|layer| layer.identifier == "Entities")
                {
                    if let Some(player_start) = entities_layer
                        .entity_instances
                        .iter()
                        .find(|entity| entity.identifier == "PlayerStart")
                    {
                        return Ok((
                            Vec2::new(player_start.px[0] as f32, player_start.px[1] as f32),
                            level.iid.clone(),
                        ));
                    }
                }
            }
        }
        Err("Could not find PlayerStart entity in any level".into())
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

            let tile_custom_data = tileset.custom_data.iter().find(|t| t.tile_id == tile.t);
            let animation = read_animation_data(tile_custom_data, tileset.c_wid);

            let tile_data = TileData {
                tileset_position: IVec2::new(tile.src[0], tile.src[1]),
                position: IVec2::new(tile.px[0], tile.px[1]),
                traversable,
                transition,
                animation,
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
            indoors: level_data.field_instances.iter().any(|field| {
                field.identifier == "indoors" && field.value.as_ref().unwrap() == true
            }),
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
                    direction: get_direction_for_destination_entity(entity),
                };

                entities.insert(entity.iid.clone(), entity_instance);
            }
        }

        Ok(entities)
    }

    pub fn get_level(&self, id: &CurrentLevelId) -> &Level {
        self.levels.get(&id.0).unwrap()
    }

    pub fn initial_level_id(&self) -> String {
        self.initial_level_id.clone()
    }
}

// TODO: refactor so that not all EntityInstace has direction field when it only applies to
// Destination entity
fn get_direction_for_destination_entity(entity: &ldtk2::EntityInstance) -> Option<Direction> {
    if entity.identifier != "Destination" {
        return None;
    }

    entity.field_instances.iter().find_map(|field| {
        if field.identifier == "Direction" {
            Some(Direction::from_str(field.value.to_owned().unwrap().as_str().unwrap()).unwrap())
        } else {
            None
        }
    })
}

/// tile custom data must contain animation data on this format:
/// animationTiles:1,2,3 (these are the tile ids in the tileset)
/// frameTime:100 (time in ms for each frame)
fn read_animation_data(
    tile_custom_data: Option<&ldtk2::TileCustomMetadata>,
    tileset_width: i64,
) -> Option<TileAnimation> {
    tile_custom_data?;
    if !tile_custom_data?.data.starts_with("animationTiles") {
        return None;
    }

    let custom_data_lines = tile_custom_data?.data.lines().collect::<Vec<&str>>();
    let animation_frames = custom_data_lines[0]
        .split_once(":")?
        .1
        .split(',')
        .map(|id| {
            let id_u32 = id.parse::<u32>().expect("Failed to parse tile id");
            tile_id_to_position(id_u32, tileset_width)
        })
        .collect::<Vec<IVec2>>();
    let frame_time =
        custom_data_lines[1].split_once(":")?.1.parse::<u64>().expect("Failed to parse frame time");

    let tile_animation = TileAnimation::new(animation_frames, Duration::from_millis(frame_time));

    Some(tile_animation)
}

fn tile_id_to_position(id: u32, tileset_width: i64) -> IVec2 {
    let x = id as i64 % tileset_width * TILE_SIZE as i64;
    let y = id as i64 / tileset_width * TILE_SIZE as i64;
    IVec2::new(x, y)
}
