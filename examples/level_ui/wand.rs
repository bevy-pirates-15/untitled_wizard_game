// Make a bundle that is given to the player
// this

// Wand gem bundle
use bevy::prelude::*;

#[derive(Component)]
struct Gem {
    gem_type: String,
}

#[derive(Component)]
struct Slot {
    slot_number: usize,
    gem: Option<Entity>,
}

#[derive(Component)]
struct Wand {
    slots: Vec<Entity>,
}

#[derive(Bundle)]
struct WandBundle {
    wand: Wand,
}

impl WandBundle {
    fn new(num_slots: usize, commands: &mut Commands) {
        let wand_entity = commands.spawn().id();
    }
}