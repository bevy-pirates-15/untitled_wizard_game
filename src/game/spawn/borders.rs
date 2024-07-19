// Gamer plan:
// Get triggered by when the level box is spawned 
// Calculate the half height/width of box
// Set variables at the top for border options

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_box_borders);
}

#[derive(Event, Debug)]
pub struct SpawnBorders;

fn spawn_box_borders (
    _trigger: Trigger<SpawnBorders>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    
}