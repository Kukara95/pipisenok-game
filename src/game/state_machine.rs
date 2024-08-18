use std::collections::HashSet;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{App, Component, info, KeyCode, Plugin, Res};
use crate::game::controls::controls::Action;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum StateMachine {
    Move(MoveState),
    Attack(AttackState),
}

impl StateMachine {
    pub fn idle() -> Self {
        StateMachine::Move(MoveState::Idle)
    }

    pub fn walk() -> Self {
        StateMachine::Move(MoveState::Walk)
    }

    pub fn run() -> Self {
        StateMachine::Move(MoveState::Run)
    }

    pub fn attack_idle() -> Self {
        StateMachine::Attack(AttackState::Idle)
    }

    pub fn attack() -> Self {
        StateMachine::Attack(AttackState::Attack)
    }

    pub fn from_move(state: MoveState) -> Self {
        StateMachine::Move(state)
    }

    pub fn from_attack(state: AttackState) -> Self {
        StateMachine::Attack(state)
    }
}

#[derive(Component, Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum MoveState {
    #[default]
    Idle,
    Walk,
    Run,
}

#[derive(Component, Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum AttackState {
    #[default]
    Idle,
    Attack
}

impl StateInterface for AttackState {
    fn transit_to_next(&mut self, actions: &HashSet<Action>) -> bool {
        match self {
            AttackState::Idle => {
                // this is the transition condition to the Walk
                if actions.contains(&Action::Attack) {
                    info!("Transiting to next state: {:?}", AttackState::Attack);
                    *self = AttackState::Attack;
                    return true
                }

                false
            },
            AttackState::Attack => {
                // this is the transition condition back to the Idle
                if !actions.contains(&Action::Attack) {
                    info!("Transiting to next state: {:?}", AttackState::Idle);
                    *self = AttackState::Idle;
                    return true
                }

                false
            },
        }
    }
}

impl StateInterface for MoveState {
    fn transit_to_next(&mut self, actions: &HashSet<Action>) -> bool {
        match self {
            MoveState::Idle => {
                // this is the transition condition to the Walk
                if actions.contains(&Action::MoveUp) || actions.contains(&Action::MoveDown) || actions.contains(&Action::MoveLeft) || actions.contains(&Action::MoveRight) {
                    info!("Transiting to next state: {:?}", MoveState::Walk);
                    *self = MoveState::Walk;
                    return true
                }

                false
            },
            MoveState::Walk => {
                // this is the transition condition back to the Idle
                if !actions.contains(&Action::MoveUp) && !actions.contains(&Action::MoveDown) && !actions.contains(&Action::MoveLeft) && !actions.contains(&Action::MoveRight) {
                    info!("Transiting to next state: {:?}", MoveState::Idle);
                    *self = MoveState::Idle;
                    return true
                }

                // this is the transition condition to the Run
                if actions.contains(&Action::Run) {
                    info!("Transiting to next state: {:?}", MoveState::Run);
                    *self = MoveState::Run;
                    return true
                }

                false
            },
            MoveState::Run => {
                if !actions.contains(&Action::Run) {
                    info!("Transiting to next state: {:?}", MoveState::Walk);
                    *self = MoveState::Walk;
                    return true
                }

                false
            },
        }
    }
}

pub trait StateInterface {
    fn transit_to_next(&mut self, actions: &HashSet<Action>) -> bool;
}
