use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{EventReader, KeyCode, Query, Transform, With};
use crate::{Player, Velocity, GROUND_LEVEL, JUMP_FORCE};

// Jump
pub fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>
) {
    for e in events.read() {
        if let Ok((mut velocity, transform)) =
            query.get_single_mut() {
            if e.state.is_pressed() && e.key_code == KeyCode::Space
                || transform.translation.y <= GROUND_LEVEL {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}
