use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

enum QueueEvent {
    TravelEvent(Vec2),
    AttackEvent,
}

#[derive(Component)]
struct CommandQueue(Vec<QueueEvent>);

#[derive(Component)]
struct Entity;

#[derive(Component)]
struct Kind(String);

#[derive(Component)]
struct Player(f32);

fn graphics_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn player_setup(mut commands: Commands, mut rapier_conf: ResMut<RapierConfiguration>) {
    rapier_conf.gravity = Vec2::ZERO;

    let sprite_size = 50.;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(sprite_size / 2., sprite_size / 2.),
        Player(50.0),
    ));
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rigid_body) in &mut player_query {
        let mut dir = Vec2::ZERO;
        if keys.pressed(KeyCode::A) {
            dir -= Vec2::new(1.0, 0.0);
        }

        if keys.pressed(KeyCode::D) {
            dir += Vec2::new(1.0, 0.0);
        }

        if keys.pressed(KeyCode::W) {
            dir += Vec2::new(0.0, 1.0);
        }

        if keys.pressed(KeyCode::S) {
            dir -= Vec2::new(0.0, 1.0);
        }

        let speed = 200.;
        let delta = dir * time.delta_seconds() * speed;
        // update linear velocity
        rigid_body.linvel = delta * player.0;
    }
}

fn main() {
    println!("run game");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "shooter2d".to_string(),
                width: 1600.0,
                height: 900.0,
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(player_setup)
        .add_startup_system(graphics_setup)
        .add_system(player_movement)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
