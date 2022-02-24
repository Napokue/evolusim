use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

const SCREEN_WIDTH: f32 = 720.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: String::from("Evolusim"),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .add_system(update)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("creatures.png");

    let chunk_size = Amount::new(2, 2);
    let tile_amount = Amount::new(8, 8);
    let tile_size = 45.;

    let map_size = MapSize(
        chunk_size.width * tile_amount.width,
        chunk_size.height * tile_amount.height,
    );

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(chunk_size.width, chunk_size.height),
            ChunkSize(tile_amount.width, tile_amount.height),
            TileSize(tile_size, tile_size),
            TextureSize(270., tile_size),
        ),
        0u16,
        0u16,
    );

    let mut i = 0;
    for x in 0..map_size.0 {
        for y in 0..map_size.1 {
            // Ignore errors for demo sake.
            let _ = layer_builder.set_tile(
                TilePos(x, y),
                Tile {
                    texture_index: 0,
                    visible: true,
                    ..Default::default()
                }
                .into(),
            );
            i += 1;
        }
    }

    map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());

    commands.entity(layer_entity).insert(LastUpdate(0.0));

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(
            0. - SCREEN_WIDTH / 2.,
            0. - SCREEN_HEIGHT / 2.,
            0.0,
        ))
        .insert(GlobalTransform::default());
}

fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut last_update_query: Query<&mut LastUpdate>,
    tile_query: Query<(Entity, &Tile, &TilePos)>,
    mut map_query: MapQuery,
) {
}

#[derive(Component)]
pub struct LastUpdate(f64);

pub struct Amount {
    pub width: u32,
    pub height: u32,
}

impl Amount {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}
