use bevy::{
    app::AppExit,
    color::palettes::css::{BURLYWOOD, CRIMSON},
    ecs::spawn::{SpawnIter, SpawnWith},
    prelude::*,
};

use crate::game::GameState;
use crate::ui::{ButtonColors, DEFAULT_TEXT_COLOR, SelectedOption, button, button_node, button_text, fullscreen_root};

/// One of the two settings that can be set through the menu. It is a resource in the app.
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    // Low,
    // Medium,
    High,
}

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub fn menu_plugin(app: &mut App) {
    app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        // Systems to handle the display settings screen
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            display_settings_menu_setup,
        )
        .add_systems(
            Update,
            (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
        )
        // Common system to all screens that handles button actions
        .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));
}

/// Font size of every menu button label.
const BUTTON_FONT_SIZE: f32 = 33.0;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
#[allow(dead_code)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
#[allow(dead_code)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
#[allow(clippy::type_complexity)]
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = ButtonColors::MENU.normal.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node { margin: UiRect::all(px(20)), ..button_node(px(300), px(65)) };
    let button_icon_node = Node {
        width: px(30),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: px(10),
        ..default()
    };

    let right_icon = asset_server.load("textures/right.png");
    let wrench_icon = asset_server.load("textures/wrench.png");
    let exit_icon = asset_server.load("textures/exitRight.png");

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        fullscreen_root(),
        children![(
            Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
            BackgroundColor(BURLYWOOD.into()),
            children![
                // Display the game name
                (
                    Text::new("Logs Of War"),
                    TextFont { font_size: 67.0, ..default() },
                    TextColor(DEFAULT_TEXT_COLOR),
                    Node { margin: UiRect::all(px(50)), ..default() },
                ),
                // Display three buttons for each action available from the main menu:
                // - new game
                // - settings
                // - quit
                (
                    button(button_node.clone(), ButtonColors::MENU),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        button_text("New Game", BUTTON_FONT_SIZE),
                    ]
                ),
                (
                    button(button_node.clone(), ButtonColors::MENU),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon), button_icon_node.clone()),
                        button_text("Settings", BUTTON_FONT_SIZE),
                    ]
                ),
                (
                    button(button_node, ButtonColors::MENU),
                    MenuButtonAction::Quit,
                    children![(ImageNode::new(exit_icon), button_icon_node), button_text("Quit", BUTTON_FONT_SIZE),]
                ),
            ]
        )],
    ));
}

fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node { margin: UiRect::all(px(20)), ..button_node(px(200), px(65)) };

    commands.spawn((
        DespawnOnExit(MenuState::Settings),
        fullscreen_root(),
        OnSettingsMenuScreen,
        children![(
            Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
            BackgroundColor(CRIMSON.into()),
            Children::spawn(SpawnIter(
                [
                    // (MenuButtonAction::SettingsDisplay, "Display"),
                    // (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        button(button_node.clone(), ButtonColors::MENU),
                        action,
                        children![button_text(text, BUTTON_FONT_SIZE)],
                    )
                })
            ))
        )],
    ));
}

fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    fn settings_button_node() -> Node {
        Node { margin: UiRect::all(px(20)), ..button_node(px(200), px(65)) }
    }

    let display_quality = *display_quality;
    commands.spawn((
        DespawnOnExit(MenuState::SettingsDisplay),
        fullscreen_root(),
        OnDisplaySettingsMenuScreen,
        children![(
            Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
            BackgroundColor(CRIMSON.into()),
            children![
                // A row, from left to right: the setting's label and its values.
                (
                    Node { align_items: AlignItems::Center, ..default() },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        // Display a label for the current setting
                        Spawn(button_text("Display Quality", BUTTON_FONT_SIZE)),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            let quality_setting = DisplayQuality::High;
                            let mut entity = parent.spawn((
                                button(Node { width: px(150), ..settings_button_node() }, ButtonColors::MENU),
                                quality_setting,
                                children![button_text(format!("{quality_setting:?}"), BUTTON_FONT_SIZE)],
                            ));
                            if display_quality == quality_setting {
                                entity.insert(SelectedOption);
                            }
                        })
                    ))
                ),
                // Display the back button to return to the settings screen
                (
                    button(settings_button_node(), ButtonColors::MENU),
                    MenuButtonAction::BackToSettings,
                    children![button_text("Back", BUTTON_FONT_SIZE)]
                )
            ]
        )],
    ));
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Briefing);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}
