use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::vec;

use bevy::asset::{ErasedAssetLoader, LoadedFolder};
use bevy::audio::CpalSample;
use bevy::prelude::KeyCode::{
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyF, KeyS, KeyW, ShiftLeft, ShiftRight,
};
use bevy::prelude::{
    default, in_state, info, App, AssetEvent, AssetServer, Assets, ButtonInput, Camera, Commands,
    Component, Entity, EventReader, EventWriter, Handle, Image, IntoSystemConfigs, KeyCode,
    NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, Sprite, SpriteBundle,
    TextureAtlas, TextureAtlasBuilder, TextureAtlasLayout, Time, Timer, TimerMode, Transform,
    TransformBundle, UVec2, Update, Vec3, With, Without,
};
use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::prelude::{
    Collider, ImpulseJoint, KinematicCharacterController, NoUserData, RapierDebugRenderPlugin,
    RapierPhysicsPlugin, RigidBody,
};

use crate::animation::animations::{AnimationClip, AnimationIndices};
use crate::game::controls::controls::{Action, ActionCommand, ActionMapping};
use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, MoveEndEvent, MoveEvent};
use crate::game::state_machine::{MoveState, StateMachine};
use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::player::assets_loading::PlayerAnimations;

const STARTING_TRANSLATION: Vec3 = Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 1.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 64.0;

const RAW_PLAYER_INITIAL_WIDTH: u32 = 52;
const RAW_PLAYER_INITIAL_HEIGHT: u32 = 52;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), (spawn_player,).chain())
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(OnEnter(AppState::MainMenu), (despawn_player))
            .add_systems(
                Update,
                (
                    player_movement,
                    stick_camera_to_player,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, animation_library: Res<PlayerAnimations>) {
    info!("Spawning Player");

    let clip = animation_library.clips.get(&(StateMachine::idle(), Direction::Zero)).unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(STARTING_TRANSLATION).with_scale(Vec3::new(3.0, 3.0, 1.0)),
            texture: clip.texture.clone(),
            ..default()
        },
        AnimationClip {
            indices: clip.indices.clone(),
            timer: clip.timer.clone(),
        },
        TextureAtlas {
            layout: clip.layout.clone(),
            index: clip.indices.first,
        },
        Collider::cuboid(
            (RAW_PLAYER_INITIAL_WIDTH / 4) as f32,
            (RAW_PLAYER_INITIAL_HEIGHT / 4) as f32,
        ),
        RigidBody::KinematicPositionBased,
        Player {},
    ));
}

pub fn player_movement(
    mut query: Query<Entity, With<Player>>,
    mut event_reader: EventReader<ActionCommand>,
    mut move_event_writer: EventWriter<MoveEvent>,
) {
    for event in event_reader.read() {
        let player_entity = query.single();
        info!("Get event: {:?}", event);

        if event.actions.contains(&Action::MoveUp) || event.actions.contains(&Action::MoveDown) || event.actions.contains(&Action::MoveLeft) || event.actions.contains(&Action::MoveRight) {
            let mut acceleration = 1.0;
            if event.actions.contains(&Action::Run) {
                acceleration = 2.0;
            }

            let move_action = MoveEvent::new(&player_entity, Direction::from_actions(event.actions.clone()), acceleration, PLAYER_SPEED);
            info!("Sending Move event: {:?}", &move_action);
            move_event_writer.send(move_action);
        }
    }
}

pub fn stick_camera_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, 2.0 * time.delta_seconds());
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}
