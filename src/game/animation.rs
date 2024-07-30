//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use std::time::Duration;

use bevy::prelude::*;

use crate::{screen::GameState, AppSet};

use super::{audio::sfx::Sfx, player_mods::movement::PlayerMovement};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<PlayerAnimation>();
    app.register_type::<EnemyAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Running)),
            (
                update_player_animation_movement,
                update_animation_atlas,
                trigger_step_sfx,
            )
                .chain()
                .in_set(AppSet::Update)
                .run_if(in_state(GameState::Running)),
        ),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_player_animation_movement(
    mut player_query: Query<(&PlayerMovement, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.0.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.0 == Vec2::ZERO {
            PlayerAnimationState::Idling
        } else {
            PlayerAnimationState::Walking
        };
        animation.update_state(animation_state);
    }
}

/// Update the animation timer.
fn update_animation_timer(
    time: Res<Time>,
    mut player_query: Query<&mut PlayerAnimation>,
    mut enemy_query: Query<&mut EnemyAnimation>,
) {
    for mut animation in &mut player_query {
        animation.update_timer(time.delta());
    }

    for mut animation in &mut enemy_query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(
    mut player_query: Query<(&PlayerAnimation, &mut TextureAtlas), Without<EnemyAnimation>>,
    mut enemy_query: Query<(&EnemyAnimation, &mut TextureAtlas), Without<PlayerAnimation>>,
) {
    for (animation, mut atlas) in &mut player_query {
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }

    for (animation, mut atlas) in &mut enemy_query {
        if animation.changed() {
            atlas.index = animation.frame as usize;
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the animation.
fn trigger_step_sfx(mut commands: Commands, mut step_query: Query<&PlayerAnimation>) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            commands.trigger(Sfx::Step);
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
    Death,
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 4;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(200);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    /// The number of walking frames.
    const WALKING_FRAMES: usize = 8;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(100);

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    /// The number of walking frames.
    #[allow(dead_code)]
    const DEATH_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const DEATH_INTERVAL: Duration = Duration::from_millis(100);

    fn death() -> Self {
        Self {
            timer: Timer::new(Self::DEATH_INTERVAL, TimerMode::Once),
            frame: 0,
            state: PlayerAnimationState::Death,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
                PlayerAnimationState::Death => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
                PlayerAnimationState::Death => *self = Self::death(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => self.frame,
            PlayerAnimationState::Walking => 8 + self.frame,
            PlayerAnimationState::Death => 16 + self.frame,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAnimation {
    timer: Timer,
    frame: u32,
}

impl EnemyAnimation {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            frame: 0,
        }
    }
    const WALKING_FRAMES: u32 = 4;

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1) % Self::WALKING_FRAMES;
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }
}
