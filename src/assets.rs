use bevy::{prelude::*, sprite::Mesh2dHandle};

#[derive(Clone)]
pub struct BulletAssets {
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
}

pub fn load_bullet_mesh(
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
