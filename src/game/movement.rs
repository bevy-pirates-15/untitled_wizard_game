//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use crate::game::input::PlayerAction;
use crate::screen::GameState;
use crate::AppSet;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use super::aiming::PlayerAim;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<PlayerMovement>();
    app.add_systems(
        Update,
        record_movement_controller
            .in_set(AppSet::RecordInput)
            .run_if(in_state(GameState::Running)),
    );

    // Apply movement based on controls.
    app.register_type::<Movement>();
    app.add_systems(
        Update,
        apply_movement
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerMovement(pub Vec2);

fn record_movement_controller(
    action_state: Res<ActionState<PlayerAction>>,
    mut movement_query: Query<&mut PlayerMovement>,
    mut aim_query: Query<&mut PlayerAim>,
) {
    // Collect directional input for movement
    let mut intent = Vec2::ZERO;
    if action_state.pressed(&PlayerAction::Move) {
        intent = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .unwrap()
            .xy()
            .clamp_length_max(1.0);
    }
    if let Ok(mut player_movement) = movement_query.get_single_mut() {
        player_movement.0 = intent
    };

    // Collect directional input for aiming
    if action_state.pressed(&PlayerAction::Look) {
        let look = action_state
            .axis_pair(&PlayerAction::Look)
            .unwrap()
            .xy()
            .normalize();
        if let Ok(mut player_aim) = aim_query.get_single_mut() {
            player_aim.0 = look
        };
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&PlayerMovement, &Movement, &mut Transform)>,
) {
    for (controller, movement, mut transform) in &mut movement_query {
        let velocity = movement.speed * controller.0;
        // this is the trouble code
        transform.translation += (velocity * time.delta_seconds()).extend(0.);
    }
}
