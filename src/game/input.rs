use bevy::app::App;
use bevy::prelude::{GamepadButtonType, MouseButton, Reflect};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{DualAxis, VirtualDPad};
use leafwing_input_manager::Actionlike;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.init_resource::<ActionState<PlayerAction>>();
    app.insert_resource(PlayerAction::default_input_map());
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
        input_map.insert(Self::Look, DualAxis::right_stick());
        input_map.insert(Self::Shoot, GamepadButtonType::RightTrigger);

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Move, VirtualDPad::arrow_keys());
        input_map.insert(Self::Shoot, MouseButton::Left);

        input_map
    }
}
