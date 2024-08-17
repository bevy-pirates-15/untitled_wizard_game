// use bevy::app::Update;
// use bevy::asset::{Asset, Assets};
// use bevy::color::LinearRgba;
// use bevy::math::Vec3;
// use bevy::prelude::{Component, GlobalTransform, Query, ResMut, TypePath};
// use bevy::render::render_resource::{AsBindGroup, ShaderRef};
// use bevy::sprite::{Material2d, Material2dPlugin};

// pub(super) fn plugin(app: &mut bevy::app::App) {
//     app.add_plugins(Material2dPlugin::<LightMaterial>::default());
//     app.add_systems(Update, update_light_material);
// }

// #[derive(Component)]
// pub struct GameLight {
//     pub radius: f32,
//     #[allow(dead_code)]
//     pub priority: u32,
// }

// #[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
// pub struct LightMaterial {
//     #[uniform(0)]
//     pub color: LinearRgba,
//     #[uniform(1)]
//     pub light_count: u32,
//     #[uniform(2)]
//     pub lights: [Vec3; 64],
// }

// impl Material2d for LightMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "light_shader.wgsl".into()
//     }
// }

// fn update_light_material(
//     mut materials: ResMut<Assets<LightMaterial>>,
//     light_sources: Query<(&GameLight, &GlobalTransform)>,
// ) {
//     let mut lights = [Vec3::ZERO; 64];
//     let mut light_count = 0;

//     for (light, transform) in light_sources.iter() {
//         lights[light_count] = transform.translation().truncate().extend(light.radius);
//         light_count += 1;
//     }

//     for (_, material) in materials.iter_mut() {
//         material.light_count = light_count as u32;
//         material.lights.copy_from_slice(&lights);
//     }
// }
