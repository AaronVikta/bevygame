use bevy::prelude::{Commands, Entity, Query, Res, Time, Transform, With};
use crate::{Life, GAME_SPEED, GROUND_EDGE};

// move life
pub fn move_life(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Life>>
){
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED/2.0 * time.delta_secs();

        if transform.translation.y < - GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}