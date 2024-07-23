use bevy::prelude::*;

use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::events::GameOver;
use crate::game::enemy::components::Enemy;
use crate::game::GameState;
use crate::game::player::components::{Lose};
use crate::game::star::components::Star;

pub fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameState::Paused => {
                next_state.set(GameState::Running);
                info!("Game resumed")
            },
            GameState::Running => {
                next_state.set(GameState::Paused);
                info!("Game paused")
            }
        }
    }
}

pub fn restart_game_on_enter(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    stars_query: Query<Entity, With<Star>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_game_state.set(GameState::Paused);
        next_app_state.set(AppState::Game);
        info!("Game restarted");

        for stars_entity in stars_query.iter() {
            commands.entity(stars_entity).despawn();
        }
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn()
        }
    }
}

pub fn handle_game_over(
    mut game_over_event: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in game_over_event.read() {
        next_app_state.set(AppState::GameOver);
        next_game_state.set(GameState::Paused);
        info!("Game is over");
        info!("Score: {}", event.score);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/menu/you_lose.png"),
                transform: Transform::from_xyz(
                    WINDOW_WIDTH / 2.0,
                    WINDOW_HEIGHT / 2.0,
                    0.0,
                ),
                ..default()
            },
            Lose {}
        ));

        commands.spawn(AudioBundle {
            source: asset_server.load("audio/annihilation-gun-sound.wav"),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

pub fn despawn_lose(
    mut commands: Commands,
    mut lose_query: Query<Entity, With<Lose>>,
) {
    if let Ok(lose_entity) = lose_query.get_single_mut() {
        commands.entity(lose_entity).despawn();
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0),
        ..default()
    });
}
