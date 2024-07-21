// Notes: I don't like using UI here, UI is unflexable for what I need

use bevy::{
    color::palettes::css::{BLACK, BLUE, BROWN, RED, WHITE},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}, tasks::futures_lite::io::Empty,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<LevelState>()
        .enable_state_scoped_entities::<LevelState>()
        .add_systems(Startup, (gem_menu, setup))
        .add_systems(Update, handle_level_action)
        .run()
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum LevelUpAction {
    Selected(Entity),
    Place(Entity),
    Continue,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum LevelState {
    #[default]
    Gem,
    Wand,
}

#[derive(Component)]
struct SelectedGem;

#[derive(Component)]
struct EmptySlot;



// TODO: Make spawn_gem be what takes arguments, make separate
// "random_gem" function that then calls spawn_gem
fn spawn_gem(commands: &mut Commands, asset_server: &AssetServer, index: i32) -> (Entity, Entity) {
    // For spawning the actual gem image
    let gem_image = asset_server.load("images/gem.png");
    let gem_image_entity = commands
        .spawn((ImageBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(45.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            image: UiImage::new(gem_image),
            ..default()
        },
        Name::new(format!("Gem{}", index)),
        ))
        .id();

    let text_entity = commands.spawn(TextBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(45.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        text: Text::from_section(
            "Cat ipsum dolor sit amet, kitty poochy yet cat slap dog in face. Eat and than sleep on your face prance along on top of the garden fence, annoy the neighbor's dog and make it bark and run in circles, scream at teh bath.",
            TextStyle {
                ..default()
            },
        ),
        ..default()
    }).id();

    (gem_image_entity, text_entity)
}

// Make observer trigger
fn gem_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };
    let gem_container = NodeBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let wand_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Percent(5.0),
                bottom: Val::Percent(5.0),
            },
            ..default()
        },
        ..default()
    };

    let ui_container_entity = commands.spawn(ui_container).id();
    let gem_container_entity = commands.spawn(gem_container).id();
    let wand_container_entity = commands.spawn(wand_container).id();

    commands
        .entity(ui_container_entity)
        .push_children(&[gem_container_entity, wand_container_entity]);

    // Idea: make a separate function that spawns the gems
    // and puts them in a table to be retrieved.
    // The gems are then visually displayed here.
    // Player then can select which gem, and then
    // place that gem into the wand
    for gem_index in 1..=3 {
        let select_gem_button = ButtonBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            background_color: Color::from(BLUE).into(),
            ..default()
        };
        let (gem_entity, text_entity) = spawn_gem(&mut commands, &asset_server, gem_index);
        
        let select_gem_button_entity = commands
            .spawn(select_gem_button)
            .insert(LevelUpAction::Selected(gem_entity))
            .id();
        commands
            .entity(gem_container_entity)
            .push_children(&[select_gem_button_entity]);
        commands
            .entity(select_gem_button_entity)
            .push_children(&[gem_entity, text_entity]);
    }

    // The Wand
    // TODO for player: Make a wand component for player that is
    // a table that stores how many "pieces" the player has, with
    // what gems are in each piece, and render that here.
    for _ in 1..=3 {
        let select_wand_button = ButtonBundle {
            style: Style {
                width: Val::Percent(10.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                },
                justify_content: JustifyContent::Center,
                ..default()
            },
            border_radius: BorderRadius::px(
                2.0,
                2.0,
                2.0,
                2.0,
            ),
            border_color: BLACK.into(),
            background_color: Color::from(BROWN).into(),
            ..default()
        };

        let slot_container = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        };

        // TODO: Read data to see if there is already a gem there, then render with 
        // spawn_gem

        let slot_container_entity = commands.spawn(slot_container).id();

        let select_wand_button_entitiy = commands
            .spawn(select_wand_button)
            .insert(LevelUpAction::Place(slot_container_entity))
            .push_children(&[slot_container_entity])
            .id();

        commands.entity(wand_container_entity).push_children(&[select_wand_button_entitiy]);
        // If this were real, I would search for the gem here
        // then push as a child of "select_wand_button"
        // for now, I will use "spawn_gem"
        // Having a gem tied to the wand will tag the slot
        // as in use, or have a query to see if there is a gem
        // in wand already?
        // let (gem_entity, _) = spawn_gem(&mut commands, &asset_server);
        // commands.entity(select_wand_button_entitiy).push_children(&[gem_entity]);

    }
}


fn handle_level_action(
    mut commands: Commands,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut button_query: InteractionQuery<&LevelUpAction>,
    selected_gem_query: Query<(Entity, &Name), With<SelectedGem>>,
) {
    for (interaction, action) in &mut button_query.iter_mut() {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                // Entity in selected is the physcial image entity
                LevelUpAction::Selected(gem_entity) => {
                    info!("Selected");
                    // Check if player has already selected anything
                    if let Ok((selected_gem_entity, name)) = selected_gem_query.get_single() {
                        info!("{} is the currently selected gem", name);
                        commands.entity(selected_gem_entity).remove::<SelectedGem>();
                    }
                    commands.entity(*gem_entity).insert(SelectedGem);
                    //TODO: Change color of selected gem button
                    info!("current gem selected: {}", gem_entity);
                },
                LevelUpAction::Place(slot) => {
                    // Check to see if a gem is selected
                    if let Ok((selected_gem_entity, _)) = selected_gem_query.get_single() {
                        // Clear children
                        commands.entity(*slot).despawn_descendants();
                        // Place the selected gem into the wand
                        commands.entity(*slot).push_children(&[selected_gem_entity]);
                        // probably here we put a component in the slot to be read?
                        // or in a player table for later rendering
                        commands.entity(selected_gem_entity).remove::<SelectedGem>();
                        // TODO: make it so that no other gems can be selected/ placed
                        
                    }
                },
                // Probably here we save the position of slots to be read when spell casting
                LevelUpAction::Continue => next_level_state.set(LevelState::Wand),
            }
        }
    }
}

type InteractionQuery<'w, 's, T> = Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;
