use bevy::color::Color;
use bevy::prelude::{BackgroundColor, Changed, Interaction, NextState, Query, ResMut, With};

use crate::ui::main_menu::components::PlayButton;
use crate::ui::main_menu::styles::{HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::AppState;

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::NONE);
            }
        }
    }
}
