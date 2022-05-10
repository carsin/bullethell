use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("tiles.png");
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
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

    layer_builder.set_all(TileBundle::default());

    //  build layer
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);

    //  spawn map
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128., -128., 0.))
        .insert(GlobalTransform::default());
}

fn remove_map(mut commands: Commands, keyboard_input: Res<Input<KeyCode>,>) {
    
}
