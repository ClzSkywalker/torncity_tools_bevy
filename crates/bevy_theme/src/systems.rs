use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;
use bevy_text::prelude::TextColor;
use super::resource::Theme;
use super::theme::ThemeColors;
use super::component::*;

pub fn apply_theme_to_components(
    theme: Res<Theme>,
    mut background_query: Query<(&ThemedBackground, &mut BackgroundColor)>,
    mut border_query: Query<(&ThemedBorder, &mut BorderColor)>,
    mut text_query: Query<(&ThemedText, &mut TextColor)>,
    mut state_query: Query<(&ThemedState, &mut BackgroundColor), Without<ThemedBackground>>,
    mut primary_button_query: Query<&mut BackgroundColor, (With<ThemedPrimaryButton>, Without<ThemedBackground>, Without<ThemedState>)>,
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

pub(crate) fn on_insert_background(
    insert: On<Insert, ThemedBackground>,
    theme: Res<Theme>,
    mut query: Query<(&ThemedBackground, &mut BackgroundColor)>,
) {
    if let Ok((themed_bg, mut bg_color)) = query.get_mut(insert.entity) {
        bg_color.0 = themed_bg.get_color(theme.colors());
    }
}

pub(crate) fn on_insert_border(
    insert: On<Insert, ThemedBorder>,
    theme: Res<Theme>,
    mut query: Query<(&ThemedBorder, &mut BorderColor)>,
) {
    if let Ok((themed_border, mut border_color)) = query.get_mut(insert.entity) {
        let color = themed_border.get_color(theme.colors());
        border_color.set_all(color);
    }
}

pub(crate) fn on_insert_text(
    insert: On<Insert, ThemedText>,
    theme: Res<Theme>,
    mut query: Query<(&ThemedText, &mut TextColor)>,
) {
    if let Ok((themed_text, mut text_color)) = query.get_mut(insert.entity) {
        text_color.0 = themed_text.get_color(theme.colors());
    }
}

pub(crate) fn on_insert_state(
    insert: On<Insert, ThemedState>,
    theme: Res<Theme>,
    mut query: Query<(&ThemedState, &mut BackgroundColor)>,
) {
    if let Ok((themed_state, mut bg_color)) = query.get_mut(insert.entity) {
        bg_color.0 = themed_state.get_color(theme.colors());
    }
}

pub(crate) fn on_insert_primary_button(
    insert: On<Insert, ThemedPrimaryButton>,
    theme: Res<Theme>,
    mut query: Query<&mut BackgroundColor, With<ThemedPrimaryButton>>,
) {
    if let Ok(mut bg_color) = query.get_mut(insert.entity) {
        bg_color.0 = theme.colors().primary;
    }
}

pub fn get_theme_colors(theme: &Theme) -> &ThemeColors {
    theme.colors()
}
