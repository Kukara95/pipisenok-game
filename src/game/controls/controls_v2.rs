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

// Определяем структуру команды, содержащую список действий
struct Command {
    actions: Vec<Action>,
}

// Ресурс для хранения сопоставления клавиш и действий
#[derive(Resource, Default, Debug)]
struct ActionMapping {
    map: HashMap<KeyCode, Action>,
}

// Компонент, представляющий текущую очередь команд
#[derive(Default)]
struct CommandQueue {
    queue: VecDeque<Command>,
    is_busy: bool,
}


