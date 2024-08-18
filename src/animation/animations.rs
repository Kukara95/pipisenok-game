use bevy::prelude::{App, Component, in_state, IntoSystemConfigs, Plugin, Query, Res, Time, Timer, Update};
use bevy::sprite::TextureAtlas;

use crate::AppState;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, animate.run_if(in_state(AppState::Game)));
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Default)]
pub struct AnimationClip {
    pub indices: AnimationIndices,
    pub timer: Timer,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut AnimationClip)>
) {
    for (mut atlas, mut clip) in query.iter_mut() {
        clip.timer.tick(time.delta());

        if clip.timer.finished() {
            atlas.index = match atlas.index {
                idx if idx < clip.indices.first => clip.indices.first,
                idx if idx < clip.indices.last => idx + 1,
                _ => clip.indices.first
            };
        }
    }
}
