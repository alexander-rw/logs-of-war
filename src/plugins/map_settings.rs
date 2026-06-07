use bevy::prelude::*;

use crate::resources::{
    DEFAULT_TEXT_COLOR,
    game_state::{GameState, GameStateEvent},
    map_selection::MapSelection,
};

pub fn map_setting_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::PreGame), map_setting_setup).add_systems(
        Update,
        (start_game_button_system, dropdown_toggle_system, dropdown_option_system).run_if(in_state(GameState::PreGame)),
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

fn map_setting_setup(mut commands: Commands) {
    commands.insert_resource(MapSelection::default());

    // Note for C# developers: `.with_children()` is the Bevy builder pattern for
    // attaching child entities. We use it here (instead of the `children![]` macro)
    // because we need to loop over `MapSelection::all_variants()` to build the
    // dropdown list dynamically.
    commands
        .spawn((
            DespawnOnExit(GameState::PreGame),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.10)),
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
                width: Val::Px(240.0),
                ..default()
            })
            .with_children(|container| {
                // Label above the dropdown
                container.spawn((
                    Text::new("Map Selection"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
                ));

                // Dropdown trigger button — initial label comes from the enum
                container
                    .spawn((
                        Button,
                        DropdownButton,
                        Node {
                            width: Val::Px(240.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::horizontal(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.22)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            // `label()` gives us "Hills", "Testing Area", etc. — not the
                            // raw `Debug` format which would show "TestingArea".
                            Text::new(MapSelection::default().label()),
                            TextFont { font_size: 24.0, ..default() },
                            TextColor(DEFAULT_TEXT_COLOR),
                            DropdownLabel,
                        ));
                        btn.spawn((
                            Text::new("v"),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
                        ));
                    });

                // Dropdown option list — populated from the enum, hidden by default
                container
                    .spawn((
                        DropdownList,
                        Visibility::Hidden,
                        Node { flex_direction: FlexDirection::Column, width: Val::Px(240.0), ..default() },
                        BackgroundColor(Color::srgb(0.13, 0.13, 0.16)),
                    ))
                    .with_children(|list| {
                        // `all_variants()` returns a `&'static [MapSelection]` — a
                        // compile-time constant slice with no heap allocation.
                        // In debug builds this includes TestingArea; release builds omit it.
                        for &variant in MapSelection::all_variants() {
                            list.spawn((
                                Button,
                                DropdownOption(variant),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(44.0),
                                    align_items: AlignItems::Center,
                                    padding: UiRect::horizontal(Val::Px(12.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.13, 0.13, 0.16)),
                            ))
                            .with_children(|option| {
                                option.spawn((
                                    Text::new(variant.label()),
                                    TextFont { font_size: 22.0, ..default() },
                                    TextColor(DEFAULT_TEXT_COLOR),
                                ));
                            });
                        }
                    });
            });

            // Begin button
            root.spawn((
                Button,
                StartButton,
                Node {
                    width: Val::Px(240.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.20, 0.45, 0.20)),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("Begin"),
                    TextFont { font_size: 32.0, ..default() },
                    TextColor(DEFAULT_TEXT_COLOR),
                ));
            });
        });
}

// --- Systems ---

/// Query filter type alias to avoid clippy `type_complexity` lint.
// Note for C# developers: Bevy query types can get verbose. A `type` alias is
// Rust's equivalent of a C# `using` type alias — purely a compile-time rename.
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
    map_selection: Res<MapSelection>,
    mut game_state_writer: MessageWriter<GameStateEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state_writer.write(GameStateEvent::MapSelected(*map_selection));
        }
    }
}
