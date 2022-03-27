use bevy::prelude::*;

pub const GUN_GLOCK: Gun = Gun {
    name: "Glock-18",
    rate: 19.0,
    damage: 1.1,
    spread: 7.0,
};


#[derive(Component)]
pub struct Gun {
    pub name: &'static str,
    pub rate: f32,
    pub damage: f32,
    pub spread: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub dir: Vec2,
    pub angle: f32,
}

pub fn update_bullets(mut query: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation.x += (bullet.dir.x * bullet.speed) * time.delta_seconds();
        transform.translation.y += (bullet.dir.y * bullet.speed) * time.delta_seconds();
    }
}
