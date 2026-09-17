//! Shared building blocks for every screen: the root node, buttons, and the
//! one system that colours them.

use bevy::prelude::*;

/// Text colour used on every screen unless a screen overrides it.
pub const DEFAULT_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// Registers the button colouring, which runs on every screen.
pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, button_system);
}

/// Marks the button that holds the current value of a setting.
#[derive(Component)]
pub struct SelectedOption;

/// The four colours a button shows, by interaction and selection.
#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    /// Idle and not selected.
    pub normal: Color,
    /// Under the pointer, and not selected.
    pub hovered: Color,
    /// Held down, or idle and selected.
    pub pressed: Color,
    /// Under the pointer, and selected.
    pub hovered_selected: Color,
}

impl ButtonColors {
    /// The grey-to-green scheme the menu screens use.
    pub const MENU: Self = Self {
        normal: Color::srgb(0.15, 0.15, 0.15),
        hovered: Color::srgb(0.25, 0.25, 0.25),
        pressed: Color::srgb(0.35, 0.75, 0.35),
        hovered_selected: Color::srgb(0.25, 0.65, 0.25),
    };
}

/// A node that fills the window and centres its children.
///
/// Spread it to change a field: `Node { row_gap: px(24), ..fullscreen_root() }`.
pub fn fullscreen_root() -> Node {
    Node {
        width: percent(100),
        height: percent(100),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

/// A button-sized node that centres its label.
///
/// Spread it to change a field: `Node { margin: UiRect::all(px(20)), ..button_node(px(300), px(65)) }`.
pub fn button_node(width: Val, height: Val) -> Node {
    Node { width, height, justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }
}

/// A clickable button that [`button_system`] colours.
///
/// The caller adds the label and any action marker as its own children and
/// components.
pub fn button(node: Node, colors: ButtonColors) -> impl Bundle {
    (Button, node, BackgroundColor(colors.normal), colors)
}

/// A button label in the default text colour.
pub fn button_text(label: impl Into<String>, font_size: f32) -> impl Bundle {
    (Text::new(label), TextFont { font_size, ..default() }, TextColor(DEFAULT_TEXT_COLOR))
}

/// Query of every button whose interaction changed this frame.
type ButtonQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static ButtonColors, &'static mut BackgroundColor, Option<&'static SelectedOption>),
    (Changed<Interaction>, With<Button>),
>;

/// Repaints a button when the pointer enters, leaves, or presses it.
fn button_system(mut interaction_query: ButtonQuery) {
    for (interaction, colors, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => colors.pressed.into(),
            (Interaction::Hovered, Some(_)) => colors.hovered_selected.into(),
            (Interaction::Hovered, None) => colors.hovered.into(),
            (Interaction::None, None) => colors.normal.into(),
        }
    }
}
