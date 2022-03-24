use bevy::{
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_tilemap::prelude::*;

mod map;
mod util;

const PLAYER_SPEED: f32 = 300.;
const PLAYER_SIZE: f32 = 20.;
const BULLET_SPEED: f32 = 1000.;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Bullet {
    speed: f32,
    dir: Vec2,
    angle: f32,
}

#[derive(Clone)]
struct BulletAssets {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

struct BulletFireEvent {
    pos: Vec2,
    dir: Vec2,
    angle: f32,
}

#[derive(Component)]
struct MainCamera;

fn update_player(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    windows: Res<Windows>, // does this need to be retrieved every update?
    mut write_bullet: EventWriter<BulletFireEvent>,
    mut p_query: Query<(&Player, &mut Transform)>,
    c_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for (player, mut p_transform) in p_query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            p_transform.translation.y += player.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::A) {
            p_transform.translation.x -= player.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::S) {
            p_transform.translation.y -= player.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::D) {
            p_transform.translation.x += player.speed * time.delta_seconds();
        }

        if mouse.just_pressed(MouseButton::Left) || keys.pressed(KeyCode::Space) {
            // get camera info and transform, assuming only 1 camera entity
            let (camera, camera_transform) = c_query.single();
            // get camera's display window
            let window = windows.get(camera.window).unwrap();
            // cursor click within window
            if let Some(click_pos) = window.cursor_position() {
                // get position in world of click
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (click_pos / window_size) * 2.0 - Vec2::ONE;
                // matrix for undoing the projection and camera transform
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix.inverse();
                // use it to convert ndc to world-space coordinates
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
                // reduce it to a 2D value
                let world_pos: Vec2 = world_pos.truncate();
                let player_pos = p_transform.translation.truncate();
                let diff = player_pos - world_pos;
                let angle = f32::atan2(diff.y, diff.x);
                let dir = (world_pos - player_pos).normalize();
                println!(
                    "fire event:\n player_pos: {}\n click_pos: {}\n angle: {}\n",
                    player_pos, world_pos, angle
                );

                // send fire event
                write_bullet.send(BulletFireEvent {
                    pos: Vec2::new(p_transform.translation.x, p_transform.translation.y)
                        + (dir * PLAYER_SIZE),
                    dir,
                    angle: angle + std::f32::consts::FRAC_PI_2,
                });
            }
        }
    }
}

fn load_bullet_mesh(
    // startup system: load render info for bullet
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(BulletAssets {
        mesh: meshes
            .add(Mesh::from(shape::Quad {
                size: Vec2::new(2., 10.),
                flip: false,
            }))
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 0.1))),
    });
}

fn update_bullets(mut query: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation.x += (bullet.dir.x * bullet.speed) * time.delta_seconds();
        transform.translation.y += (bullet.dir.y * bullet.speed) * time.delta_seconds();
    }
}

// spawns bullet on bulletfireevent send
fn spawn_bullet(
    mut commands: Commands,
    assets: Res<BulletAssets>,
    mut listen_bullet: EventReader<BulletFireEvent>,
) {
    for fire in listen_bullet.iter() {
        commands
            .spawn()
            .insert(Bullet {
                speed: BULLET_SPEED,
                angle: fire.angle,
                dir: fire.dir,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::default()
                    .with_translation(Vec3::new(fire.pos.x, fire.pos.y, 0.0))
                    .with_rotation(Quat::from_rotation_z(fire.angle)),
                ..Default::default()
            });
    }
}
fn spawn_camera(mut commands: Commands) {
    // startup system: spawn perspective camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn update_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut transform, mut projection) in query.iter_mut() {
        let mut dir = Vec3::ZERO;
        if keys.pressed(KeyCode::Left) {
            dir -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::Right) {
            dir += Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::Up) {
            dir += Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.pressed(KeyCode::Down) {
            dir -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.pressed(KeyCode::Z) {
            projection.scale += 0.03;
        }

        if keys.pressed(KeyCode::X) {
            projection.scale -= 0.03;
        }

        if projection.scale < 0.5 {
            projection.scale = 0.5;
        }

        // restore Z values to ensure camera's view of layers isn't messed up after modifying camera
        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * dir * PLAYER_SPEED;
        transform.translation.z = z;
    }
}

fn spawn_player(
    // startup system: init player
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn()
        .insert(Player {
            speed: PLAYER_SPEED,
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(PLAYER_SIZE)),
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 1.0))),
            ..Default::default()
        });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bullethell prototype 'corpseCo' !dwmf".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(load_bullet_mesh)
        .add_startup_system(map::spawn_tilemap)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_event::<BulletFireEvent>()
        .add_system(util::set_texture_filters_to_nearest)
        .add_system(spawn_bullet)
        .add_system(update_bullets)
        .add_system(update_player)
        .add_system(update_camera)
        .run();
}
