use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SPEED: f32 = 100.;

#[derive(Component)]
struct Player;

fn update_player(keys: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<(&Player, &mut Transform)>) {
    for (player, mut transform) in query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            transform.translation.y += SPEED * time.delta_seconds();
        }
        
        if keys.pressed(KeyCode::S) {
            transform.translation.y -= SPEED * time.delta_seconds();
        }
        
        if keys.pressed(KeyCode::A) {
            transform.translation.x -= SPEED * time.delta_seconds();
        }
        
        if keys.pressed(KeyCode::D) {
            transform.translation.x += SPEED * time.delta_seconds();
        }
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn()
        .insert(Player)
        .insert_bundle(MaterialMesh2dBundle { 
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(50.)),
            material: materials.add(ColorMaterial::from(Color::rgb(0.1, 1., 0.1))),
            ..Default::default()
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game !dwmfloat".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(update_player)
        .run();
}
