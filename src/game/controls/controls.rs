use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};

use bevy::input::ButtonInput;
use bevy::prelude::{App, Event, EventWriter, in_state, IntoSystemConfigs, KeyCode, Plugin, Res, Resource, Update, Vec3, info, Commands, OnEnter};
use bevy::prelude::KeyCode::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyF, KeyS, KeyW, ShiftLeft};
use crate::AppState;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActionMapping>()
            .add_event::<ActionCommand>()
            .add_systems(OnEnter(AppState::Loading), setup_player_controls)
            .add_systems(Update, handle_input.run_if(in_state(AppState::Game)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Idle,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Attack,
    Run,
}

#[derive(Event, Debug)]
pub struct ActionCommand {
    pub actions: HashSet<Action>,
}

#[derive(Resource, Default)]
pub struct ActionMapping {
    pub binding: HashMap<KeyCode, Action>,
}

#[derive(Resource, Default)]
pub struct CommandQueue {
    queue: VecDeque<ActionCommand>,
    is_busy: bool,
}

pub fn setup_player_controls(mut commands: Commands) {
    commands.insert_resource(
        ActionMapping {
            binding: HashMap::from([
                (KeyW, Action::MoveUp),
                (KeyA, Action::MoveLeft),
                (KeyS, Action::MoveDown),
                (KeyD, Action::MoveRight),
                (ArrowUp, Action::MoveUp),
                (ArrowLeft, Action::MoveLeft),
                (ArrowDown, Action::MoveDown),
                (ArrowRight, Action::MoveRight),
                (ShiftLeft, Action::Run),
                (KeyF, Action::Attack),
            ])
        }
    );
}

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    action_mapping: Res<ActionMapping>,
    mut event_writer: EventWriter<ActionCommand>,
) {
    // we need to collect intersections between pressed keys and keys in the action binding
    let pressed_actions = keyboard_input
        .get_pressed()
        .collect::<HashSet<_>>()
        .intersection(&action_mapping.binding.keys().collect::<HashSet<_>>())
        .map(|key| action_mapping.binding[key])
        .collect::<HashSet<_>>();

    if !pressed_actions.is_empty() {
        info!("Pressed actions: {:?}", pressed_actions);
        event_writer.send(
            ActionCommand {
                actions: pressed_actions
            }
        );
    }
}
