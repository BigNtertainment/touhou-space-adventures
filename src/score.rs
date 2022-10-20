use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(load_score));
    }
}

fn load_score(mut commands: Commands) {
    commands.insert_resource(Score::new(0));
}

pub struct Score {
    score: i32,
}

impl Score {
    fn new(starting_score: i32) -> Score {
        Score {
            score: starting_score,
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn add_to_score(&mut self, score_to_add: i32) {
        self.score += score_to_add;
    }
}