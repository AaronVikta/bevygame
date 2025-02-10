use bevy::prelude::*;
use crate::{GROUND_LEVEL, GROUND_EDGE, OBSTACLE_COLOR, OBSTACLE_SIZE, Obstacle, ObstacleSpawningTimer};
use bevy_prng::WyRand;
use bevy::sprite::Anchor;
use bevy_rand::global::GlobalEntropy;
use rand_core::RngCore;


// spawn a new obstacle once the timer has counted down zero
pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    mut rng: GlobalEntropy<WyRand>
){
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        let obstacle_x = GROUND_EDGE;
        let obstacle_y = GROUND_LEVEL + (rng.next_u32() % 50) as f32;
        commands.spawn((Obstacle,
                        Sprite{
                            color:OBSTACLE_COLOR,
                            custom_size: Some(OBSTACLE_SIZE),
                            anchor: Anchor::BottomCenter,
                            ..default()
                        },
                        Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
        ));
    }
}