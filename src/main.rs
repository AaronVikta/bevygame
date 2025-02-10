// use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
// use bevy::sprite::Anchor;
use crate::GameState::{GameOver, InGame};
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use crate::apply_gravity::apply_gravity;
use crate::check_health::check_health;
use crate::detect_collision::{detect_collision, life_collision};
use crate::jump::jump;
use crate::move_life::move_life;
use crate::move_obstacles::move_obstacles;
use crate::player_movement::player_movement;
use crate::render_health_info::render_health_info;
use crate::setup::setup;
use crate::spawn_life::spawn_life;
use crate::spawn_obstacles::spawn_obstacles;

mod apply_gravity;
mod check_health;
mod detect_collision;
mod jump;
mod move_obstacles;
mod player_movement;
mod render_health_info;
mod setup;
mod spawn_obstacles;
mod spawn_life;
mod move_life;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .insert_resource(ObstacleSpawningTimer(
            Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_resource(LifeSpawningTimer(
            Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_state(InGame)
        .add_systems(Update, (jump, apply_gravity, player_movement)
            .run_if(in_state(InGame)))
        .add_systems(Update, (spawn_obstacles, move_obstacles, spawn_life,move_life, detect_collision, life_collision, render_health_info, check_health)
            .run_if(in_state(InGame)))
        .add_systems(OnEnter(GameOver), game_over)
        .run();
}

//region Constants
const GAME_SPEED: f32 = 200.0;
const JUMP_FORCE: f32 = 600.0;
const GRAVITY: f32 = -800.0;
const PLAYER_X: f32 = -300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
const PLAYER_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const SPAWN_INTERVAL: f32 = 1.0;
const GROUND_LEVEL: f32 = -100.0;
const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const LIFE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const LIFE_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct HealthInfo;

#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);

#[derive(Resource)]
struct LifeSpawningTimer(Timer);
#[derive(Component)]
struct Life;


fn game_over(mut commands: Commands) {
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(10.),
        right: Val::Percent(10.),
        top: Val::Percent(15.),
        bottom: Val::Percent(15.),
        justify_content: JustifyContent::Center,
        ..default()
    },))
        .with_children(|builder| {
            builder.spawn((
                Text("GAME OVER".to_string()),
                TextFont::from_font_size(160.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
        });
}