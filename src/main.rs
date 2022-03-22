use bevy::{
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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
    // rot: Vec2,
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

fn update_player(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    windows: Res<Windows>, // does this need to be retrieved every update?
    mut write_bullet: EventWriter<BulletFireEvent>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            transform.translation.y += player.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::A) {
            transform.translation.x -= player.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::S) {
            transform.translation.y -= player.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::D) {
            transform.translation.x += player.speed * time.delta_seconds();
        }

        if mouse.just_pressed(MouseButton::Left) || keys.pressed(KeyCode::Space) {
            let window = windows.get_primary().unwrap();
            if let Some(click) = window.cursor_position() {
                // cursor click within window
                let player_pos = transform.translation.truncate();
                // calculate click based on origin at middle (consistent with shooter)
                let rel_click_pos = vec2(
                    click.x - (window.width() / 2.),
                    click.y - (window.height() / 2.),
                );
                let diff = player_pos - rel_click_pos;
                let angle = f32::atan2(diff.y, diff.x);
                let dir = (rel_click_pos - player_pos).normalize();
                println!(
                    "fire event:\n player_pos: {}\n click_pos: {}\n angle: {}\n",
                    player_pos, rel_click_pos, angle
                );

                // send fire event
                write_bullet.send(BulletFireEvent {
                    pos: Vec2::new(transform.translation.x, transform.translation.y)
                        + (dir * PLAYER_SIZE),
                    dir,
                    angle: angle + std::f32::consts::FRAC_PI_2,
                });
            }
        }
    }
}

fn spawn_player(
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
            material: materials.add(ColorMaterial::from(Color::rgb(0.1, 1., 0.1))),
            ..Default::default()
        });
}

fn load_bullet_mesh(
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

fn update_bullets(mut query: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation.x += (bullet.dir.x * bullet.speed) * time.delta_seconds();
        transform.translation.y += (bullet.dir.y * bullet.speed) * time.delta_seconds();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bullethell prototype 'corpseCo' !dwmf".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_bullet_mesh)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_event::<BulletFireEvent>()
        .add_system(update_player)
        .add_system(spawn_bullet)
        .add_system(update_bullets)
        .run();
}
