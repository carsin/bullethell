// Systems for Player Entity, Camera
use crate::assets;
use crate::game;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PLAYER_SPEED: f32 = 300.;
const PLAYER_SIZE: f32 = 20.;
const BULLET_SPEED: f32 = 1000.;

#[derive(Component)]
pub struct Player {
    speed: f32,
    gun: game::Gun,
}

#[derive(Component)]
pub struct MainCamera {
    locked: bool,
}

// Bevy Event
pub struct PlayerBulletFireEvent {
    pub pos: Vec2,
    pub dir: Vec2,
    pub angle: f32,
}

// startup system spawn camera
pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera { locked: true })
        .insert_bundle(UiCameraBundle::default());
}

pub fn spawn_player(
    // startup system: init player
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("player-idle.png");
    commands
        .spawn()
        .insert(Player {
            gun: game::GUN_GLOCK,
            speed: PLAYER_SPEED,
        })
        // TODO: convert to sprite
        // .insert_bundle(MaterialMesh2dBundle {
        //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        //     transform: Transform::default().with_scale(Vec3::splat(PLAYER_SIZE)),
        //     material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 1.0))),
        //     ..Default::default()
        .insert_bundle(SpriteSheetBundle {
            transform: Transform::default().with_scale(Vec3::splat(PLAYER_SIZE)),
            ..Default::default()
        });
}

// Bevy EventListener: spawns bullet on bulletfireevent
pub fn spawn_bullet(
    mut commands: Commands,
    assets: Res<assets::BulletAssets>,
    mut fire_event: EventReader<PlayerBulletFireEvent>,
) {
    for bullet in fire_event.iter() {
        commands
            .spawn()
            .insert(game::Bullet {
                speed: BULLET_SPEED,
                dir: bullet.dir,
                angle: bullet.angle,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::default()
                    .with_translation(Vec3::new(bullet.pos.x, bullet.pos.y, 0.0))
                    .with_rotation(Quat::from_rotation_z(bullet.angle)),
                ..Default::default()
            });
    }
}

pub fn player_control(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut write_bullet: EventWriter<PlayerBulletFireEvent>,
    mut p_query: Query<(&Player, &mut Transform)>,
    c_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for (player, mut p_transform) in p_query.iter_mut() {
        let mut dir = Vec3::ZERO;
        if keys.pressed(KeyCode::A) {
            dir -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::D) {
            dir += Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::W) {
            dir += Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.pressed(KeyCode::S) {
            dir -= Vec3::new(0.0, 1.0, 0.0);
        }

        // transform mesh without damaging z value
        let z = p_transform.translation.z;
        p_transform.translation += time.delta_seconds() * dir * player.speed;
        p_transform.translation.z = z;

        if mouse.just_pressed(MouseButton::Left) || keys.just_pressed(KeyCode::Space) {
            // get camera info and transform, assuming only 1 camera entity
            let (camera, camera_transform) = c_query.single();
            // get camera's display window
            let window = windows.get_primary().unwrap();
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
                write_bullet.send(PlayerBulletFireEvent {
                    pos: Vec2::new(p_transform.translation.x, p_transform.translation.y)
                        + (dir * PLAYER_SIZE),
                    dir,
                    angle: angle + std::f32::consts::FRAC_PI_2,
                });
            }
        }
    }
}

pub fn camera_control(
    mut query: ParamSet<(
        Query<(&mut Transform, &mut OrthographicProjection, &mut MainCamera), With<MainCamera>>,
        Query<(&Player, &Transform), Changed<Transform>>,
    )>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    //  store player pos
    let player_pos = if let Ok(p_transform) = query.p1().get_single() {
        Some(p_transform.1.translation)
    } else {
        None
    };

    // controls
    for (mut transform, mut projection, mut cam) in query.p0().iter_mut() {
        let mut dir = Vec3::ZERO;
        if keys.pressed(KeyCode::Left) || (cam.locked && keys.pressed(KeyCode::A)) {
            dir -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::Right) || (cam.locked && keys.pressed(KeyCode::D)) {
            dir += Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::Up) || (cam.locked && keys.pressed(KeyCode::W)) {
            dir += Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.pressed(KeyCode::Down) || (cam.locked && keys.pressed(KeyCode::S)) {
            dir -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.just_pressed(KeyCode::Y) && player_pos != None {
            // toggle lock
            cam.locked = !cam.locked;
            // center cam on player
            let z = transform.translation.z;
            transform.translation = player_pos.unwrap();
            transform.translation.z = z;
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

        // transform camera translation without damaging z value
        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * dir * PLAYER_SPEED;
        transform.translation.z = z;
    }
}
