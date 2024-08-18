use std::collections::HashSet;

use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;
use rand::{random, Rng};

use crate::AppState;
use crate::game::controls::controls::Action;
use crate::game::game::GameState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<MoveEndEvent>()
            .add_systems(
            Update,
            (update_position)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Event, Debug)]
pub struct MoveEvent {
    pub entity: Entity,
    pub direction: Direction,
    pub acceleration: f32,
    pub speed: f32,
}

#[derive(Event, Debug)]
pub struct MoveEndEvent {
    pub entity: Entity,
}

impl MoveEvent {
    pub fn new(entity: &Entity, direction: Direction, acceleration: f32, speed: f32) -> Self {
        Self {
            entity: *entity,
            direction,
            acceleration,
            speed,
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, Hash)]
pub enum Direction {
    #[default]
    Zero,
    Random,
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up) => true,
            (Direction::Down, Direction::Down) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            (Direction::UpRight, Direction::UpRight) => true,
            (Direction::UpLeft, Direction::UpLeft) => true,
            (Direction::DownRight, Direction::DownRight) => true,
            (Direction::DownLeft, Direction::DownLeft) => true,
            (Direction::Random, Direction::Random) => true,
            (Direction::Zero, Direction::Zero) => true,
            _ => false,
        }
    }
}

impl Direction {
    pub const DIRECTIONS: [Self; 8] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpRight,
        Direction::UpLeft,
        Direction::DownRight,
        Direction::DownLeft,
    ];

    const NEG_X_AXES: [Self; 3] = [
        Direction::Left,
        Direction::UpLeft,
        Direction::DownLeft
    ];

    pub fn from_actions(actions: HashSet<Action>) -> Direction {
        if actions.contains(&Action::MoveUp) && actions.contains(&Action::MoveRight) {
            Direction::UpRight
        } else if actions.contains(&Action::MoveDown) && actions.contains(&Action::MoveLeft) {
            Direction::DownLeft
        } else if actions.contains(&Action::MoveDown) && actions.contains(&Action::MoveRight) {
            Direction::DownRight
        } else if actions.contains(&Action::MoveUp) && actions.contains(&Action::MoveLeft) {
            Direction::UpLeft
        } else if actions.contains(&Action::MoveUp) {
            Direction::Up
        } else if actions.contains(&Action::MoveDown) {
            Direction::Down
        } else if actions.contains(&Action::MoveLeft) {
            Direction::Left
        } else if actions.contains(&Action::MoveRight) {
            Direction::Right
        } else {
            Direction::Zero
        }
    }

    pub fn vec_from_actions(actions: Vec<Action>) -> Vec3 {
        if actions.contains(&Action::MoveUp) && actions.contains(&Action::MoveRight) {
            return Direction::Up.get_direction_vec() + Direction::Right.get_direction_vec();
        }

        if actions.contains(&Action::MoveDown) && actions.contains(&Action::MoveLeft) {
            return Direction::Down.get_direction_vec() + Direction::Left.get_direction_vec();
        }

        if actions.contains(&Action::MoveDown) && actions.contains(&Action::MoveRight) {
            return Direction::Down.get_direction_vec() + Direction::Right.get_direction_vec();
        }

        if actions.contains(&Action::MoveUp) && actions.contains(&Action::MoveLeft) {
            return Direction::Up.get_direction_vec() + Direction::Left.get_direction_vec();
        }

        if actions.contains(&Action::MoveUp) {
            return Direction::Up.get_direction_vec();
        }

        if actions.contains(&Action::MoveDown) {
            return Direction::Down.get_direction_vec();
        }

        if actions.contains(&Action::MoveLeft) {
            return Direction::Left.get_direction_vec();
        }

        if actions.contains(&Action::MoveRight) {
            return Direction::Right.get_direction_vec();
        }

        Direction::Zero.get_direction_vec()
    }

    pub fn is_neg_x_axes(&self) -> bool {
        Self::NEG_X_AXES.contains(self)
    }

    pub fn get_direction_vec(&self) -> Vec3 {
        match self {
            Direction::Up => Vec3::Y,
            Direction::Down => Vec3::NEG_Y,
            Direction::Right => Vec3::X,
            Direction::Left => Vec3::NEG_X,
            Direction::UpRight => Vec3::new(1.0, 1.0, 0.0),
            Direction::UpLeft => Vec3::new(-1.0, 1.0, 0.0),
            Direction::DownRight => Vec3::new(1.0, -1.0, 0.0),
            Direction::DownLeft => Vec3::new(-1.0, -1.0, 0.0),
            Direction::Random => {
                let mut rng = rand::thread_rng();
                let generated =
                    Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
                info!("Generated: {:?}", generated);

                generated
            }
            _ => Vec3::ZERO,
        }
    }
}

pub fn update_position(
    mut event_reader: EventReader<MoveEvent>,
    mut transform_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    for mut move_event in event_reader.read() {
        let (mut transform) = transform_query.get_mut(move_event.entity).unwrap();
        info!("Get event: {:?}", &move_event);

        let mut direction = move_event.direction.get_direction_vec();

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * move_event.speed * move_event.acceleration * time.delta_seconds();
    }
}
