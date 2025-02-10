use bevy::math::Vec3;
use bevy::prelude::{default, Camera2d, Commands, Sprite, Text, Transform};
use bevy::sprite::Anchor;
use crate::{Health, HealthInfo, Player, Velocity, GROUND_COLOR, GROUND_EDGE, GROUND_LEVEL, GROUND_SIZE, PLAYER_COLOR, PLAYER_SIZE, PLAYER_X};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let initial_health = 8;
    // Player
    commands
        .spawn((
            Player,
            Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
            Velocity(Vec3::ZERO),
            Health(initial_health)
        ));

    commands.spawn((
        HealthInfo,
        Text::new(format!("Health: {}", initial_health))
    )
    );

    // Ground
    commands.spawn((
        Sprite {
            color: GROUND_COLOR,
            custom_size: Some(GROUND_SIZE),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-GROUND_EDGE, GROUND_LEVEL, 0.0)
    ));
}
