use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod assets;
mod controller;
mod game;
mod map;
mod util;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "prototype !dwmf".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(assets::load_bullet_mesh)
        .add_startup_system(controller::spawn_camera)
        .add_startup_system(controller::spawn_player)
        .add_startup_system(map::spawn_map)
        .add_event::<controller::PlayerBulletFireEvent>()
        .add_system(util::set_texture_filters_to_nearest)
        .add_system(controller::spawn_bullet)
        // update
        .add_system(game::update_bullets)
        .add_system(controller::player_control)
        .add_system(controller::camera_control)
        .add_system(bevy::input::system::exit_on_esc_system) // prototyping builtin
        .run();
}
