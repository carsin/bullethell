use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

// startup system: create game map
pub fn spawn_tilemap(
    // startup system: load render info for bullet
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
) {
    let tile_textures: Handle<Image> = asset_server.load("tiles.png");
    let map_size = MapSize(20, 20);

    let layer_settings = LayerSettings::new(
        map_size,
        ChunkSize(32, 32),
        TileSize(16.0, 16.0),
        TextureSize(96.0, 16.0),
    );

    let (mut layer_builder, layer_0_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);

    layer_builder.set_all(Tile::default().into());

    map_query.build_layer(&mut commands, layer_builder, tile_textures);

    let tile_textures = asset_server.load("anim.png");

    let map_size = MapSize(map_size.0 / 2, map_size.1 / 2);
    let layer_settings = LayerSettings::new(
        map_size,
        ChunkSize(32, 32),
        TileSize(32.0, 32.0),
        TextureSize(32.0, 448.0),
    );
    let (mut layer_builder, layer_1_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 1u16);

    let mut random = thread_rng();

    for _ in 0..1000 {
        let position = TilePos(
            random.gen_range(0..map_size.0 * 32),
            random.gen_range(0..map_size.1 * 32),
        );
        let _ = layer_builder.set_tile(
            position,
            Tile {
                texture_index: 0,
                ..Default::default()
            }
            .into(),
        );

        if let Ok(entity) = layer_builder.get_tile_entity(&mut commands, position) {
            commands
                .entity(entity)
                .insert(GPUAnimated::new(0, 13, 0.95));
        }
    }

    map_query.build_layer(&mut commands, layer_builder, tile_textures);

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_0_entity);
    map.add_layer(&mut commands, 1u16, layer_1_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-5120.0, -5120.0, 0.0))
        .insert(GlobalTransform::default());
}
