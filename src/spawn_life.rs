use bevy::prelude::*;
use crate::{GROUND_LEVEL, GROUND_EDGE, LIFE_COLOR, LIFE_SIZE, ObstacleSpawningTimer, Life};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy::sprite::Anchor;
use rand_core::RngCore;



pub fn spawn_life(
    mut commands: Commands,
mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    time: Res<Time>,
    mut rng: GlobalEntropy<WyRand>) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        let life_x = GROUND_EDGE;
        let life_y = GROUND_LEVEL + (rng.next_u32() % 50) as f32;

        commands.spawn((Life,
        Sprite{
            color: LIFE_COLOR,
            custom_size: Some(LIFE_SIZE),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(life_x, life_y, 0.0),
        ));
    }
}