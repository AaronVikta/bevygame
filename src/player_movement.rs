use bevy::prelude::{Query, Res, Time, Transform, With};
use crate::{Player, Velocity, GROUND_LEVEL};

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    for (mut transform, mut velocity)
    in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}
