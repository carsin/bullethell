use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

enum QueueEvent {
    TravelEvent,
}

#[derive(Component)]
struct Entity {
    command_queue: Vec<QueueEvent>,
}

#[derive(Component)]
struct Kind(String);

#[derive(Component)]
struct Player(f32);

fn player_setup(mut commands: Commands, mut rapier_conf: ResMut<RapierConfiguration>) {
    rapier_conf.gravity = Vec2::ZERO;
    commands.spawn(Camera2dBundle::default());
    
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
        Collider::ball(sprite_size / 2.),
        Player(50.0),
    ));
}

fn player_movement(input: Res<Input<KeyCode>>, mut player_query: Query<(&Player, &mut Velocity)>) {
    for (player, mut rigid_body) in &mut player_query {
        let up = input.pressed(KeyCode::W);
        let down = input.pressed(KeyCode::S);
        let left = input.pressed(KeyCode::A);
        let right = input.pressed(KeyCode::D);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut delta = Vec2::new(x_axis as f32, y_axis as f32);
        if delta != Vec2::ZERO {
            delta /= delta.length();
        }
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
        .add_system(player_movement)
        .run();
}
