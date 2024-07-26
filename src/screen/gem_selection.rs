use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    color::palettes::css::BLUE,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use palette::*;
use prelude::{InteractionPalette, InteractionQuery};

use crate::{
    game::spell_system::{
        storage::{RebuildWand, SpellInventory, SpellPool},
        SpellComponent,
    },
    ui::*,
};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GemSelection), gem_menu);
    app.add_systems(
        Update,
        (
            handle_gem_select_action,
            handle_gem_placement_action,
            handle_mouse_scroll,
        )
            .run_if(in_state(GameState::GemSelection)),
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

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

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

fn gem_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pool: ResMut<SpellPool>,
    spell_inventory: Res<SpellInventory>,
) {
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
            height: Val::Percent(30.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let scrolling_container = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_self: AlignSelf::Center,
            width: Val::Percent(50.),
            height: Val::Percent(25.),
            margin: UiRect::all(Val::Percent(0.5)),
            overflow: Overflow::clip_x(),
            ..default()
        },
        background_color: Color::srgb(0.10, 0.10, 0.10).into(),
        ..default()
    };

    let moving_panel = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let continue_button = ButtonBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Percent(0.5)),
            ..default()
        },
        border_color: BorderColor(NODE_BACKGROUND.1),
        border_radius: BorderRadius::all(Val::Percent(10.)),
        background_color: BackgroundColor(NODE_BACKGROUND.0),
        ..default()
    };

    // Only ui_container has to be scoped, as everything else
    // is a child of it
    let ui_container_entity = commands
        .spawn(ui_container)
        .insert(StateScoped(GameState::GemSelection))
        .id();

    let gem_container_entity = commands.spawn(gem_container).id();
    let scrolling_container_entity = commands.spawn(scrolling_container).id();
    let moving_panel_entity = commands
        .spawn(moving_panel)
        .insert(ScrollingList::default())
        .insert(AccessibilityNode(NodeBuilder::new(Role::List)))
        .id();

    let continue_button_entity = commands
        .spawn(continue_button)
        .insert(InteractionPalette {
            none: NODE_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        })
        .insert(GemPlaceButtonSound)
        .insert(LevelUpAction::Placed)
        .id();

    commands.entity(ui_container_entity).push_children(&[
        gem_container_entity,
        scrolling_container_entity,
        continue_button_entity,
    ]);

    commands
        .entity(scrolling_container_entity)
        .push_children(&[moving_panel_entity]);

    // For rending spells that the player currently has
    for spell in spell_inventory.spells.iter() {
        let spell_container = NodeBundle {
            style: Style {
                width: Val::Percent(35.),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            ..default()
        };
        // clippy is just completely wrong here, everything you do here is wrong
        #[allow(clippy::useless_format)]
        let spell_name = TextBundle::from_section(
            format!("{}", spell.data.get_name()),
            TextStyle {
                font_size: *UiScale(40.),
                ..default()
            },
        );

        let spell_container_entity = commands.spawn(spell_container).id();

        let spell_name_entity = commands
            .spawn(spell_name)
            .insert(AccessibilityNode(NodeBuilder::new(Role::ListItem)))
            .id();

        commands
            .entity(moving_panel_entity)
            .push_children(&[spell_container_entity]);

        commands
            .entity(spell_container_entity)
            .push_children(&[spell_name_entity]);
    }

    // For rendering the random gems on screen
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

fn handle_gem_placement_action(
    mut commands: Commands,
    mut button_query: InteractionQuery<&LevelUpAction>,
    mut spell_inventory: ResMut<SpellInventory>,
    mut next_gamestate: ResMut<NextState<GameState>>,
    selected_gem_query: Query<&SpellComponent, With<SelectedGem>>,
) {
    for (interaction, action) in &mut button_query.iter_mut() {
        if matches!(interaction, Interaction::Pressed) && action == &LevelUpAction::Placed {
            if let Ok(spell) = selected_gem_query.get_single() {
                spell_inventory.push_spell(spell.clone());
                commands.trigger(RebuildWand);
                next_gamestate.set(GameState::Running);
            }
        }
    }
}

fn handle_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_width = list_node.size().x;
            let container_width = query_node.get(parent.get()).unwrap().size().x;

            let max_scroll = (items_width - container_width).max(0.);

            let delta_x = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.x * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.x,
            };

            scrolling_list.position += delta_x;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.left = Val::Px(scrolling_list.position);
        }
    }
}
