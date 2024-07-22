use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{DualAxis, VirtualDPad};
use leafwing_input_manager::Actionlike;
use crate::AppSet;
use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.init_resource::<ActionState<PlayerAction>>();
    app.insert_resource(PlayerAction::default_input_map());

    app.add_systems(Update, player_mouse_look.in_set(AppSet::RecordInput));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Look,
    Shoot,
}

impl PlayerAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        // input_map.insert(Self::Look, DualAxis::right_stick());
        input_map.insert(Self::Shoot, GamepadButtonType::RightTrigger);

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        // input_map.insert(Self::Look, VirtualDPad::arrow_keys());
        input_map.insert(Self::Shoot, MouseButton::Left);
        input_map.insert(Self::Shoot, KeyCode::Space);

        input_map
    }
}

// Function is not working as intended
fn player_mouse_look(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut action_state: ResMut<ActionState<PlayerAction>>,
) {
    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let Ok(player_transform) = player_query.get_single() else { return; };
    let window = window_query
        .get_single()
        .expect("Need a single primary window");

    // Many steps can fail here, so we'll wrap in an option pipeline
    // First check if the cursor is in window
    // Then check if the ray intersects the plane defined by the player
    // Then finally compute the point along the ray to look at
    let player_position = player_transform.translation;
    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .and_then(|ray| {
            Some(ray).zip(ray.intersect_plane(player_position, InfinitePlane3d::new(Vec3::Z)))
        })
        .map(|(ray, p)| ray.get_point(p))
    {
        println!("Looking at {}", p);
        let diff = (p - player_position).xy();
        println!("Diff: {}", diff);
        if diff.length_squared() > 0.01 {
            // Get the mutable action data to set the axis
            let action_data = action_state.action_data_mut_or_default(&PlayerAction::Look);

            // Flipping y sign here to be consistent with gamepad input.
            // We could also invert the gamepad y-axis
            action_data.axis_pair = Some(DualAxisData::new(diff.x, -diff.y));

            // Press the look action, so we can check that it is active
            action_state.press(&PlayerAction::Look);
        }
    }
}
