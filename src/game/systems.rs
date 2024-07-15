use bevy::audio::AudioBundle;
use bevy::input::ButtonInput;
use bevy::prelude::{AssetServer, Commands, default, Entity, EventReader, info, KeyCode, NextState, PlaybackSettings, Query, Res, ResMut, SpriteBundle, State, Transform, Vec2, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;
use crate::AppState;
use crate::events::GameOver;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::NUMBER_OF_ENEMIES;
use crate::game::GameState;
use crate::game::player::components::{Lose, Player};
use crate::game::score::components::Score;
use crate::game::star::components::Star;
use crate::game::star::NUMBER_OF_STARS;

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
    window_query: Query<&Window, With<PrimaryWindow>>,
    stars_query: Query<Entity, With<Star>>,
    enemy_query: Query<Entity, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let window = window_query.get_single().unwrap();

    if keyboard_input.just_pressed(KeyCode::Enter) {
        for stars_entity in stars_query.iter() {
            commands.entity(stars_entity).despawn();
        }
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn()
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    0.0,
                ),
                texture: asset_server.load("sprites/characters/shinobi/Idle-cropped.png"),
                ..default()
            },
            Player {},
            Score {
                value: 0
            }
        ));

        for _ in 0..NUMBER_OF_ENEMIES {
            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/balls/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random(), random()).normalize(),
                },
            ));
        }

        for _ in 0..NUMBER_OF_STARS {
            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/balls/star.png"),
                    ..default()
                },
                Star {},
            ));
        }

        next_state.set(GameState::Paused)
    }
}

pub fn handle_game_over(
    mut game_over_event: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for event in game_over_event.read() {
        next_app_state.set(AppState::GameOver);
        next_game_state.set(GameState::Paused);
        info!("Score: {}", event.score);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/menu/you_lose.png"),
                transform: Transform::from_xyz(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    0.0,
                ),
                ..default()
            },
            Lose {}
        ));

        commands.spawn(AudioBundle {
            source: asset_server.load("audio/annihilation-gun-sound.wav"),
            settings: PlaybackSettings::ONCE,
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