use bevy::prelude::{App, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::AppState;
use systems::interactions::interact_with_play_button;
use systems::layout::{despwan_main_menu, spawn_main_menu};

mod systems;
pub mod components;
mod styles;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despwan_main_menu)
            .add_systems(Update, interact_with_play_button.run_if(in_state(AppState::MainMenu)));
    }
}