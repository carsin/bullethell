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

// setup player
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
        Player(50.0),
        CommandQueue(Vec::new()),
        // rapier
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(sprite_size / 2., sprite_size / 2.),
    ));
}

// setup another entity
// https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
fn entity_setup(mut commands: Commands) {
    let sprite_size = 20.;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.0, 0.0),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            ..Default::default()
        },
        Entity,
        Kind("Example".to_string()),
        CommandQueue(vec!(QueueEvent::TravelEvent(Vec2::new(500., 500.)))),
        // rapier
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(sprite_size / 2., sprite_size / 2.),
    ));
}

fn move_entity(rigid_body: &mut Velocity, dest: &Vec2, pos: &Transform) -> bool {
    let speed = 10.;
    if pos.translation.x + rigid_body.linvel.x > dest.x { // destination below entity
        rigid_body.linvel.x = speed;
    } else if pos.translation.x + rigid_body.linvel.x < dest.x { // destination above entity
        rigid_body.linvel.x = -speed;
    } else if pos.translation.x + rigid_body.linvel.x == dest.x {
        return true;
    }
    
    if pos.translation.y + rigid_body.linvel.y > dest.y { // destination below entity
        rigid_body.linvel.y = speed;
    } else if pos.translation.y + rigid_body.linvel.y < dest.y { // destination above entity
        rigid_body.linvel.y = -speed;
    } else if pos.translation.y + rigid_body.linvel.y == dest.y {
        return true;
    }
    false
}

fn event_handler(mut commands: Commands, mut query: Query<(&mut Entity, &mut CommandQueue, &Transform, &mut Velocity)>) {
    let mut complete = false;
    for (mut entity, mut queue, pos, mut rigid_body) in &mut query {
        if !queue.0.is_empty() {
            println!("handling event");
            // handle the event in the queue
            match &queue.0[0] {
                QueueEvent::TravelEvent(dest) => {
                    complete = move_entity(&mut rigid_body, dest, pos);
                },
                AttackEvent => {
                    todo!();
                },
            }
            // determine if event has been completed and can be removed from queue
            if complete {
                println!("event complete");
                queue.0.remove(0);
            }
        }
        
    }
}

// move player with keyboard
fn player_controls(
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
        .add_startup_system(entity_setup)
        .add_startup_system(graphics_setup)
        .add_system(player_controls)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
