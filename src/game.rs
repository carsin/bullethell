use bevy::prelude::*;

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
