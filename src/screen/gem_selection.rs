use bevy::{color::palettes::css::BLUE, prelude::*};

use crate::{
    game::spell_system::{storage::SpellPool, SpellComponent},
    ui::*,
};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GemSelection), gem_menu);
    app.add_systems(
        Update,
        handle_gem_select_action.run_if(in_state(GameState::GemSelection)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum LevelUpAction {
    Selected,
    Placed,
}

#[derive(Component)]
struct SelectedGem;

// TODO: Make spawn_gem be what takes arguments, make separate
// "random_gem" function that then calls spawn_gem
fn spawn_gem(
    commands: &mut Commands,
    asset_server: &AssetServer,
    index: i32,
    pool: &ResMut<SpellPool>,
) -> (Entity, Entity, SpellComponent) {
    // For spawning the actual gem image
    let gem_image = asset_server.load("images/gem.png");
    let gem = pool.get_random_spell_component().clone();
    let gem_description = gem.data.get_desc();
    let gem_image_entity = commands
        .spawn((
            ImageBundle {
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

    let text_entity = commands
        .spawn(TextBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(45.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            text: Text::from_section(gem_description, TextStyle { ..default() }),
            ..default()
        })
        .id();

    (gem_image_entity, text_entity, gem)
}

fn gem_menu(mut commands: Commands, asset_server: Res<AssetServer>, pool: ResMut<SpellPool>) {
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

    // Only ui_container has to be scoped, as everything else
    // is a child of it
    let ui_container_entity = commands
        .spawn(ui_container)
        .insert(StateScoped(GameState::GemSelection))
        .id();
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
        let (gem_entity, text_entity, spell) =
            spawn_gem(&mut commands, &asset_server, gem_index, &pool);

        let select_gem_button_entity = commands
            .spawn(select_gem_button)
            .insert(GemPickUpButtonSound)
            .insert(LevelUpAction::Selected)
            .insert(spell)
            .id();
        commands
            .entity(gem_container_entity)
            .push_children(&[select_gem_button_entity]);
        commands
            .entity(select_gem_button_entity)
            .push_children(&[gem_entity, text_entity]);
    }
}

fn handle_gem_select_action(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &LevelUpAction, &SpellComponent, Entity),
        Changed<Interaction>,
    >,
    selected_gem_query: Query<(Entity, &SpellComponent), With<SelectedGem>>,
) {
    for (interaction, action, _spell, entity) in &mut button_query.iter_mut() {
        if matches!(interaction, Interaction::Pressed) && action == &LevelUpAction::Selected {
            // Entity in selected is the physcial image entity
            if let Ok((entity, _)) = selected_gem_query.get_single() {
                commands.entity(entity).remove::<SelectedGem>();
            }
            commands.entity(entity).insert(SelectedGem);
        }
    }
}

// fn handle_gem_placement_action(
//     mut commands: Commands,
//     mut button_query: InteractionQuery<&LevelUpAction>,
//     selected_gem_query: Query<(&SpellComponent), With<SelectedGem>>,
// ) {
//     for (interaction, action) in &mut button_query.iter_mut() {
//         if matches!(interaction, Interaction::Pressed) {
//             match action {
//                 LevelUpAction::Placed => {
//                     if let Ok(spell) = selected_gem_query.get_single() {

//                     }
//                 },
//                 _ => ()
//             }
//         }
//     }
// }
