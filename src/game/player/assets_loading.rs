use crate::animation::animations::AnimationIndices;
use crate::game::controls::controls::{Action, ActionMapping};
use crate::game::movement::movement::Direction;
use crate::game::state_machine::StateMachine;
use crate::AppState;
use bevy::app::Update;
use bevy::asset::{AssetEvent, AssetServer, Assets, Handle, LoadedFolder};
use bevy::log::info;
use bevy::math::UVec2;
use bevy::prelude::KeyCode::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyF, KeyS, KeyW, ShiftLeft};
use bevy::prelude::{in_state, Commands, EventReader, Image, IntoSystemConfigs, NextState, OnEnter, Plugin, Res, ResMut, Resource, TextureAtlasLayout, Timer, TimerMode};
use std::collections::HashMap;

pub struct AssetsLoadingPlugin;

impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<PlayerAnimations>()
            .init_resource::<PlayerAssets>()
            .add_systems(OnEnter(AppState::Loading), load_player_assets)
            .add_systems(OnEnter(AppState::AnimationLoading), setup_animations)
            .add_systems(
                Update,
                check_textures.run_if(in_state(AppState::Loading)),
            );
    }
}

#[derive(Resource, Default, Debug)]
pub struct PlayerAssets(Handle<LoadedFolder>);

#[derive(Resource, Default, Debug)]
pub struct PlayerAnimations {
    pub clips: HashMap<(StateMachine, Direction), PlayerAnimationClip>
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerAnimationClip {
    pub indices: AnimationIndices,
    pub timer: Timer,
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

pub fn load_player_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    info!("Loading Player assets");
    commands.insert_resource(PlayerAssets(asset_server.load_folder("sprites/characters/knight-basic")));
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    player_assets: Res<PlayerAssets>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&player_assets.0) {
            next_state.set(AppState::AnimationLoading);
            info!("Player assets loaded!");
        }
    }
}

fn setup_animations(
    asset_server: Res<AssetServer>,
    textures: Res<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    //TODO: think how to reduce this big guy
    let idle_down: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_down.png").unwrap();
    let idle_down_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_down_left.png").unwrap();
    let idle_down_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_down_right.png").unwrap();
    let idle_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_left.png").unwrap();
    let idle_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_right.png").unwrap();
    let idle_up: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_up.png").unwrap();
    let idle_up_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_up_left.png").unwrap();
    let idle_up_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Idle/Knight_Idle_up_right.png").unwrap();

    let attack_down: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_down.png").unwrap();
    let attack_down_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_down_left.png").unwrap();
    let attack_down_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_down_right.png").unwrap();
    let attack_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_left.png").unwrap();
    let attack_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_right.png").unwrap();
    let attack_up: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_up.png").unwrap();
    let attack_up_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_up_left.png").unwrap();
    let attack_up_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Attack/Knight_Attack_up_right.png").unwrap();

    let walk_down: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_down.png").unwrap();
    let walk_down_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_down_left.png").unwrap();
    let walk_down_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_down_right.png").unwrap();
    let walk_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_left.png").unwrap();
    let walk_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_right.png").unwrap();
    let walk_up: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_up.png").unwrap();
    let walk_up_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_up_left.png").unwrap();
    let walk_up_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Walk/Knight_Walk_up_right.png").unwrap();

    let run_down: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_down.png").unwrap();
    let run_down_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_down_left.png").unwrap();
    let run_down_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_down_right.png").unwrap();
    let run_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_left.png").unwrap();
    let run_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_right.png").unwrap();
    let run_up: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_up.png").unwrap();
    let run_up_left: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_up_left.png").unwrap();
    let run_up_right: Handle<Image> = asset_server.get_handle("sprites/characters/knight-basic/Run/Knight_Run_up_right.png").unwrap();

    let idle_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(textures.get(idle_down.id()).unwrap().width() / 5), 5, 4, None, None
    );
    let walk_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(textures.get(walk_down.id()).unwrap().width() / 4), 4, 3, None, None
    );
    let run_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(textures.get(run_down.id()).unwrap().width() / 3), 3, 3, None, None
    );
    let attack_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(textures.get(attack_down.id()).unwrap().width() / 4), 4, 4, None, None
    );

    let idle_indices = AnimationIndices { first: 0, last: 16 };
    let walk_indices = AnimationIndices { first: 0, last: 10 };
    let run_indices = AnimationIndices { first: 0, last: 7 };
    let attack_indices = AnimationIndices { first: 0, last: 14 };

    let idle_down_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_down.clone(),
    };
    let idle_down_right_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_down_right.clone(),
    };
    let idle_down_left_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_down_left.clone(),
    };
    let idle_right_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_right.clone(),
    };
    let idle_left_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_left.clone(),
    };
    let idle_up_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_up.clone(),
    };
    let idle_up_right_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_up_right.clone(),
    };
    let idle_up_left_clip = PlayerAnimationClip {
        indices: idle_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(idle_layout.clone()),
        texture: idle_up_left.clone(),
    };

    let walk_down_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_down.clone(),
    };
    let walk_down_right_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_down_right.clone(),
    };
    let walk_down_left_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_down_left.clone(),
    };
    let walk_right_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_right.clone(),
    };
    let walk_left_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_left.clone(),
    };
    let walk_up_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_up.clone(),
    };
    let walk_up_right_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_up_right.clone(),
    };
    let walk_up_left_clip = PlayerAnimationClip {
        indices: walk_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(walk_layout.clone()),
        texture: walk_up_left.clone(),
    };

    let run_down_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_down.clone(),
    };
    let run_down_right_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_down_right.clone(),
    };
    let run_down_left_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_down_left.clone(),
    };
    let run_right_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_right.clone(),
    };
    let run_left_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_left.clone(),
    };
    let run_up_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_up.clone(),
    };
    let run_up_right_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_up_right.clone(),
    };
    let run_up_left_clip = PlayerAnimationClip {
        indices: run_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(run_layout.clone()),
        texture: run_up_left.clone(),
    };

    let attack_down_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_down.clone(),
    };
    let attack_down_right_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_down_right.clone(),
    };
    let attack_down_left_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_down_left.clone(),
    };
    let attack_right_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_right.clone(),
    };
    let attack_left_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_left.clone(),
    };
    let attack_up_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_up.clone(),
    };
    let attack_up_right_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_up_right.clone(),
    };
    let attack_up_left_clip = PlayerAnimationClip {
        indices: attack_indices.clone(),
        timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        layout: layouts.add(attack_layout.clone()),
        texture: attack_up_left.clone(),
    };

    let player_clips = HashMap::from([
        ((StateMachine::idle(), Direction::Zero), idle_down_clip.clone()),
        ((StateMachine::idle(), Direction::Down), idle_down_clip.clone()),
        ((StateMachine::idle(), Direction::DownRight), idle_down_right_clip.clone()),
        ((StateMachine::idle(), Direction::DownLeft), idle_down_left_clip.clone()),
        ((StateMachine::idle(), Direction::Right), idle_right_clip.clone()),
        ((StateMachine::idle(), Direction::Left), idle_left_clip.clone()),
        ((StateMachine::idle(), Direction::Up), idle_up_clip.clone()),
        ((StateMachine::idle(), Direction::UpRight), idle_up_right_clip.clone()),
        ((StateMachine::idle(), Direction::UpLeft), idle_up_left_clip.clone()),

        ((StateMachine::walk(), Direction::Down), walk_down_clip.clone()),
        ((StateMachine::walk(), Direction::DownRight), walk_down_right_clip),
        ((StateMachine::walk(), Direction::DownLeft), walk_down_left_clip),
        ((StateMachine::walk(), Direction::Right), walk_right_clip),
        ((StateMachine::walk(), Direction::Left), walk_left_clip),
        ((StateMachine::walk(), Direction::Up), walk_up_clip),
        ((StateMachine::walk(), Direction::UpRight), walk_up_right_clip),
        ((StateMachine::walk(), Direction::UpLeft), walk_up_left_clip),
        ((StateMachine::walk(), Direction::Zero), walk_down_clip),

        ((StateMachine::run(), Direction::Down), run_down_clip.clone()),
        ((StateMachine::run(), Direction::DownRight), run_down_right_clip),
        ((StateMachine::run(), Direction::DownLeft), run_down_left_clip),
        ((StateMachine::run(), Direction::Right), run_right_clip),
        ((StateMachine::run(), Direction::Left), run_left_clip),
        ((StateMachine::run(), Direction::Up), run_up_clip),
        ((StateMachine::run(), Direction::UpRight), run_up_right_clip),
        ((StateMachine::run(), Direction::UpLeft), run_up_left_clip),
        ((StateMachine::run(), Direction::Zero), run_down_clip),

        ((StateMachine::attack_idle(), Direction::Zero), idle_down_clip.clone()),
        ((StateMachine::attack_idle(), Direction::Down), idle_down_clip.clone()),
        ((StateMachine::attack_idle(), Direction::DownRight), idle_down_right_clip),
        ((StateMachine::attack_idle(), Direction::DownLeft), idle_down_left_clip),
        ((StateMachine::attack_idle(), Direction::Right), idle_right_clip),
        ((StateMachine::attack_idle(), Direction::Left), idle_left_clip),
        ((StateMachine::attack_idle(), Direction::Up), idle_up_clip),
        ((StateMachine::attack_idle(), Direction::UpRight), idle_up_right_clip),
        ((StateMachine::attack_idle(), Direction::UpLeft), idle_up_left_clip),

        ((StateMachine::attack(), Direction::Down), attack_down_clip.clone()),
        ((StateMachine::attack(), Direction::DownRight), attack_down_right_clip),
        ((StateMachine::attack(), Direction::DownLeft), attack_down_left_clip),
        ((StateMachine::attack(), Direction::Right), attack_right_clip),
        ((StateMachine::attack(), Direction::Left), attack_left_clip),
        ((StateMachine::attack(), Direction::Up), attack_up_clip),
        ((StateMachine::attack(), Direction::UpRight), attack_up_right_clip),
        ((StateMachine::attack(), Direction::UpLeft), attack_up_left_clip),
        ((StateMachine::attack(), Direction::Zero), attack_down_clip),
    ]);

    commands.insert_resource(PlayerAnimations { clips: player_clips });
    next_state.set(AppState::MainMenu);
}