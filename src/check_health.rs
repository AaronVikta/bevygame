use bevy::prelude::{NextState, Query, ResMut, With};
use crate::{GameState, Health, Player};
use crate::GameState::GameOver;

pub fn check_health(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if let Ok(Health(health)) = player_query.get_single() {
        if *health == 0 {
            game_state.set(GameOver);
        }
    }
}
