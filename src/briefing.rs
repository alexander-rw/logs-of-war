use bevy::prelude::*;

use crate::game::GameState;
use crate::maps::MapSelection;
use crate::ui::{ButtonColors, DEFAULT_TEXT_COLOR, button, button_node, button_text, fullscreen_root};

/// Background of the briefing screen.
const SCREEN_BACKGROUND: Color = Color::srgb(0.08, 0.08, 0.10);

/// Text colour of a caption above a control.
const CAPTION_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.6);

/// Width shared by the dropdown, its options, and the Begin button.
const CONTROL_WIDTH: Val = Val::Px(240.0);

/// Colours of the dropdown trigger and its options.
const DROPDOWN_COLORS: ButtonColors = ButtonColors {
    normal: Color::srgb(0.18, 0.18, 0.22),
    hovered: Color::srgb(0.24, 0.24, 0.30),
    pressed: Color::srgb(0.30, 0.30, 0.38),
    hovered_selected: Color::srgb(0.30, 0.30, 0.38),
};

/// Colours of one option in the open dropdown list.
const OPTION_COLORS: ButtonColors = ButtonColors {
    normal: Color::srgb(0.13, 0.13, 0.16),
    hovered: Color::srgb(0.20, 0.20, 0.25),
    pressed: Color::srgb(0.26, 0.26, 0.32),
    hovered_selected: Color::srgb(0.26, 0.26, 0.32),
};

/// Colours of the green Begin button.
const BEGIN_COLORS: ButtonColors = ButtonColors {
    normal: Color::srgb(0.20, 0.45, 0.20),
    hovered: Color::srgb(0.26, 0.55, 0.26),
    pressed: Color::srgb(0.32, 0.65, 0.32),
    hovered_selected: Color::srgb(0.32, 0.65, 0.32),
};

pub fn briefing_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Briefing), briefing_setup).add_systems(
        Update,
        (start_game_button_system, dropdown_toggle_system, dropdown_option_system)
            .run_if(in_state(GameState::Briefing)),
    );
}

// --- Components ---

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct DropdownButton;

#[derive(Component)]
struct DropdownList;

#[derive(Component)]
struct DropdownOption(MapSelection);

#[derive(Component)]
struct DropdownLabel;

// --- Setup ---

fn briefing_setup(mut commands: Commands, map_selection: Res<MapSelection>) {
    let selected_label = map_selection.label();

    // `.with_children()` rather than the `children![]` macro, because the
    // dropdown list is built by looping over `MapSelection::all_variants()`.
    commands
        .spawn((
            DespawnOnExit(GameState::Briefing),
            Node { flex_direction: FlexDirection::Column, row_gap: px(24), ..fullscreen_root() },
            BackgroundColor(SCREEN_BACKGROUND),
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("Battle Briefing"),
                TextFont { font_size: 56.0, ..default() },
                TextColor(DEFAULT_TEXT_COLOR),
            ));

            // Map dropdown container
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                width: CONTROL_WIDTH,
                ..default()
            })
            .with_children(|container| {
                // Label above the dropdown
                container.spawn((
                    Text::new("Map Selection"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(CAPTION_COLOR),
                ));

                // Dropdown trigger button — the label shows the current selection
                container
                    .spawn((
                        button(
                            Node {
                                justify_content: JustifyContent::SpaceBetween,
                                padding: UiRect::horizontal(px(12)),
                                ..button_node(CONTROL_WIDTH, px(50))
                            },
                            DROPDOWN_COLORS,
                        ),
                        DropdownButton,
                    ))
                    .with_children(|btn| {
                        btn.spawn((button_text(selected_label, 24.0), DropdownLabel));
                        btn.spawn((
                            Text::new("v"),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(CAPTION_COLOR),
                        ));
                    });

                // Dropdown option list — populated from the enum, hidden by default
                container
                    .spawn((
                        DropdownList,
                        Visibility::Hidden,
                        Node { flex_direction: FlexDirection::Column, width: CONTROL_WIDTH, ..default() },
                        BackgroundColor(OPTION_COLORS.normal),
                    ))
                    .with_children(|list| {
                        // `all_variants()` returns a `&'static [MapSelection]` — a
                        // compile-time constant slice with no heap allocation.
                        // In debug builds this includes TestingArea; release builds omit it.
                        for &variant in MapSelection::all_variants() {
                            list.spawn((
                                button(
                                    Node {
                                        justify_content: JustifyContent::FlexStart,
                                        padding: UiRect::horizontal(px(12)),
                                        ..button_node(percent(100), px(44))
                                    },
                                    OPTION_COLORS,
                                ),
                                DropdownOption(variant),
                            ))
                            .with_children(|option| {
                                option.spawn(button_text(variant.label(), 22.0));
                            });
                        }
                    });
            });

            // Begin button
            root.spawn((button(button_node(CONTROL_WIDTH, px(60)), BEGIN_COLORS), StartButton)).with_children(|btn| {
                btn.spawn(button_text("Begin", 32.0));
            });
        });
}

// --- Systems ---

/// Query filter type alias to avoid clippy `type_complexity` lint.
type DropdownOptionQuery<'w, 's> =
    Query<'w, 's, (&'static Interaction, &'static DropdownOption), (Changed<Interaction>, With<Button>)>;

fn dropdown_toggle_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<DropdownButton>)>,
    mut list_query: Query<&mut Visibility, With<DropdownList>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed
            && let Ok(mut visibility) = list_query.single_mut()
        {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

fn dropdown_option_system(
    interaction_query: DropdownOptionQuery,
    mut label_query: Query<&mut Text, With<DropdownLabel>>,
    mut list_query: Query<&mut Visibility, With<DropdownList>>,
    mut map_selection: ResMut<MapSelection>,
) {
    for (interaction, option) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *map_selection = option.0;
            if let Ok(mut label) = label_query.single_mut() {
                // Use `label()` rather than `format!("{:?}", ...)` so the display
                // string matches what the dropdown option showed ("Testing Area"
                // not "TestingArea").
                label.0 = option.0.label().to_string();
            }
            if let Ok(mut visibility) = list_query.single_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn start_game_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Battle);
        }
    }
}
