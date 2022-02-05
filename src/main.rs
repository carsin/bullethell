use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform};

const SPEED: f32 = 100.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet {
    speed: i32,
}

#[derive(Clone)]
struct BulletAssets {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

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

fn load_bullet_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(BulletAssets {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(2., 10.),
            flip: false,
        })).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.1))),
    });
}

fn spawn_bullet(mut commands: Commands, assets: Res<BulletAssets>) {
    commands.spawn()
        .insert(Bullet {
            speed: 10,
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: assets.mesh.clone(),
            material: assets.material.clone(),
            ..Default::default()
        });
}

fn update_bullets(mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation.y += bullet.speed as f32;
    }
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
        .add_startup_system(load_bullet_mesh)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_bullet)
        .add_system(update_player)
        .add_system(update_bullets)
        .run();
}
