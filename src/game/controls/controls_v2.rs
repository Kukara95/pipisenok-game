use std::collections::{HashMap, VecDeque};
use bevy::prelude::{KeyCode, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Attack,
    Run,
}

struct Command {
    actions: Vec<Action>,
}

#[derive(Resource, Default, Debug)]
struct ActionMapping {
    map: HashMap<KeyCode, Action>,
}

#[derive(Default)]
struct CommandQueue {
    queue: VecDeque<Command>,
    is_busy: bool,
}


