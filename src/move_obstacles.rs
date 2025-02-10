use bevy::prelude::{Commands, Entity, Query, Res, Time, Transform, With};
use crate::{Obstacle, GAME_SPEED, GROUND_EDGE};

// move obstacles
pub fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform),
        With<Obstacle>>
){
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        //     Remove obstacle once they're offscreen
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}