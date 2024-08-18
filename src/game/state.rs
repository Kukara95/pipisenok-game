use bevy::prelude::Component;
use crate::game::movement::movement::Direction;

pub trait State {}

#[derive(Component, Debug, Default)]
pub struct IdleState(Direction);

impl State for IdleState {}

#[derive(Component, Debug, Default)]
pub struct WalkState(Direction);

impl State for WalkState {}

#[derive(Component, Debug, Default)]
pub struct RunState(Direction);

impl State for RunState {}

#[derive(Component, Debug, Default)]
pub struct AttackState(Direction);

impl State for AttackState {}
