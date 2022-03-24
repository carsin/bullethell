use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// startup system: create game map
pub fn spawn_tilemap(
    // startup system: load render info for bullet
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
) {
    let tile_textures: Handle<Image> = asset_server.load("tiles.png");

    // create entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // create maplayer via layerbuilder entity with single layer
    let (mut layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(16., 16.),
            TextureSize(96., 16.),
        ),
        0u16,
        0u16,
    );

    // build layer; layer no longer modifiable until bevy hard sync
    layer_builder.set_all(TileBundle::default());
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, tile_textures);
    map.add_layer(&mut commands, 0u16, layer_entity);

    // spawn map entity
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128., -128., 0.))
        .insert(GlobalTransform::default());
}
