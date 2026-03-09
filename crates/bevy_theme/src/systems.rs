use super::component::*;
use super::resource::Theme;
use super::theme::ThemeColors;
use bevy_ecs::prelude::*;
use bevy_text::prelude::TextColor;
use bevy_ui::prelude::*;

pub fn apply_theme_to_components(
    theme: Res<Theme>,
    mut background_query: Query<(&ThemedBackground, &mut BackgroundColor)>,
    mut border_query: Query<(&ThemedBorder, &mut BorderColor)>,
    mut text_query: Query<(&ThemedText, &mut TextColor)>,
    mut state_query: Query<(&ThemedState, &mut BackgroundColor), Without<ThemedBackground>>,
    mut primary_button_query: Query<
        &mut BackgroundColor,
        (
            With<ThemedPrimaryButton>,
            Without<ThemedBackground>,
            Without<ThemedState>,
        ),
    >,
) {
    let colors = theme.colors();

    for (themed_bg, mut bg_color) in &mut background_query {
        bg_color.0 = themed_bg.get_color(colors);
    }

    for (themed_border, mut border_color) in &mut border_query {
        border_color.left = themed_border.get_color(colors);
        border_color.right = themed_border.get_color(colors);
        border_color.top = themed_border.get_color(colors);
        border_color.bottom = themed_border.get_color(colors);
    }

    for (themed_text, mut text_color) in &mut text_query {
        text_color.0 = themed_text.get_color(colors);
    }

    for (themed_state, mut bg_color) in &mut state_query {
        bg_color.0 = themed_state.get_color(colors);
    }

    for mut bg_color in &mut primary_button_query {
        bg_color.0 = colors.primary;
    }
}

pub fn on_change_background(
    theme: Res<Theme>,
    mut changed_themed_bg: Query<
        (&ThemedBackground, &mut BackgroundColor),
        Or<(Changed<ThemedBackground>, Added<ThemedBackground>)>,
    >,
) {
    for (themed_bg, mut bg_color) in &mut changed_themed_bg {
        bg_color.0 = themed_bg.get_color(theme.colors());
    }
}

pub fn on_change_border(
    theme: Res<Theme>,
    mut changed_themed_border: Query<
        (&ThemedBorder, &mut BorderColor),
        Or<(Changed<ThemedBorder>, Added<ThemedBorder>)>,
    >,
) {
    for (themed_border, mut border_color) in &mut changed_themed_border {
        let color = themed_border.get_color(theme.colors());
        border_color.set_all(color);
    }
}

pub fn on_change_text(
    theme: Res<Theme>,
    mut changed_themed_text: Query<
        (&ThemedText, &mut TextColor),
        Or<(Changed<ThemedText>, Added<ThemedText>)>,
    >,
) {
    for (themed_text, mut text_color) in &mut changed_themed_text {
        text_color.0 = themed_text.get_color(theme.colors());
    }
}

pub fn on_insert_text(
    theme: Res<Theme>,
    mut query: Query<(&ThemedText, &mut TextColor), Or<(Changed<ThemedText>, Added<ThemedText>)>>,
) {
    for (themed_text, mut text_color) in &mut query {
        text_color.0 = themed_text.get_color(theme.colors());
    }
}

pub(crate) fn on_change_state(
    theme: Res<Theme>,
    mut query: Query<
        (&ThemedState, &mut BackgroundColor),
        Or<(Changed<ThemedState>, Added<ThemedState>)>,
    >,
) {
    for (themed_state, mut bg_color) in &mut query {
        bg_color.0 = themed_state.get_color(theme.colors());
    }
}

pub fn on_change_button(
    theme: Res<Theme>,
    mut query: Query<
        &mut BackgroundColor,
        (
            Or<(Changed<ThemedPrimaryButton>, Added<ThemedPrimaryButton>)>,
            Without<ThemedSecondaryButton>,
        ),
    >,
    mut query_sec: Query<
        &mut BackgroundColor,
        (
            Or<(Changed<ThemedSecondaryButton>, Added<ThemedSecondaryButton>)>,
            Without<ThemedPrimaryButton>,
        ),
    >,
) {
    for mut bg_color in &mut query {
        bg_color.0 = theme.colors().primary;
    }
    for mut bg_color in &mut query_sec {
        bg_color.0 = theme.colors().secondary;
    }
}

pub fn get_theme_colors(theme: &Theme) -> &ThemeColors {
    theme.colors()
}
