use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    DefaultPlugins,
};
use rand::Rng;

const UNIT_SIZE: f32 = 20.0;
const MAX_WIDTH: i32 = 50;
const MAX_HEIGHT: i32 = 30;

const FIELD_WIDTH: f32 = UNIT_SIZE * MAX_WIDTH as f32;
const FIELD_HEIGHT: f32 = UNIT_SIZE * MAX_HEIGHT as f32;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(GameTimer(Timer::new(
            std::time::Duration::from_millis(150),
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (FIELD_WIDTH, FIELD_HEIGHT).into(),
                title: "snake game".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                update_game_timer,
                (spawn_food, update_player_direction)
                    .in_set(PlayerUpdate)
                    .after(update_game_timer),
                eat_food.after(PlayerUpdate),
                update_snake_body.after(eat_food),
            ),
        )
        .add_systems(PostUpdate, update_translation)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_xyz(-100.0, -100.0, 0.0),
            ..Default::default()
        },
        Position { x: 0, y: 0 },
        Player {
            direction: Direction::Left,
        },
    ));
}

fn update_game_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer
        .0
        .tick(std::time::Duration::from_secs_f32(time.delta_seconds()));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
struct Player {
    pub direction: Direction,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
struct PlayerUpdate;

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct Food;

fn update_player_direction(
    mut query: Query<Option<&mut Player>, (With<Player>, Without<SnakeBody>)>,
    keycode: Res<Input<KeyCode>>,
) {
    let queried_player = query.single_mut();
    if let Some(mut player) = queried_player {
        if keycode.pressed(KeyCode::Right) {
            player.direction = Direction::Right;
        }
        if keycode.pressed(KeyCode::Left) {
            player.direction = Direction::Left;
        }
        if keycode.pressed(KeyCode::Up) {
            player.direction = Direction::Up;
        }
        if keycode.pressed(KeyCode::Down) {
            player.direction = Direction::Down;
        }
    }
}

fn update_snake_body(
    timer: ResMut<GameTimer>,
    mut player_query: Query<
        Option<(&mut Player, &mut Position)>,
        (With<Player>, Without<SnakeBody>),
    >,
    mut body_query: Query<&mut Position, (With<SnakeBody>, Without<Player>)>,
) {
    let tuple = player_query.single_mut().unwrap();
    let player = tuple.0;
    let mut position = tuple.1;

    let mut prev_translation = position.clone();
    if !timer.0.finished() {
        return;
    }

    match player.direction {
        Direction::Right => {
            if position.x < MAX_WIDTH / 2 - 1 {
                position.x += 1;
            }
        }
        Direction::Left => {
            if position.x >= -MAX_WIDTH / 2 + 1 {
                position.x -= 1;
            }
        }
        Direction::Up => {
            if position.y < MAX_HEIGHT / 2 - 1 {
                position.y += 1;
            }
        }
        Direction::Down => {
            if position.y >= -MAX_HEIGHT / 2 + 1 {
                position.y -= 1;
            }
        }
    }
    for mut body in body_query.iter_mut() {
        let prev = body.clone();
        body.x = prev_translation.x;
        body.y = prev_translation.y;
        prev_translation = prev;
    }
}

fn update_translation(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            (pos.x as f32 + 0.5) * UNIT_SIZE,
            (pos.y as f32 + 0.5) * UNIT_SIZE,
            0.0,
        );
    }
}

fn spawn_food(mut commands: Commands, mut query: Query<Option<&Position>, With<Food>>) {
    if query.iter_mut().next().is_some() {
        return;
    }
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-100.0, -100.0, 0.0),
            ..Default::default()
        },
        Food,
        Position {
            x: rand::thread_rng().gen_range(-MAX_WIDTH / 2..MAX_WIDTH / 2),
            y: rand::thread_rng().gen_range(-MAX_HEIGHT / 2..MAX_HEIGHT / 2),
        },
    ));
}

fn eat_food(
    mut commands: Commands,
    player_query: Query<&Position, (With<Player>, Without<SnakeBody>)>,
    food_query: Query<(Entity, &Position), With<Food>>,
) {
    let player_pos = player_query.single();

    for food in food_query.iter() {
        if player_pos.x == food.1.x && player_pos.y == food.1.y {
            commands.entity(food.0).despawn();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(-100.0, -100.0, 0.0),
                    ..Default::default()
                },
                SnakeBody,
                Position { x: -100, y: -100 },
            ));
        }
    }
}

#[derive(Resource)]
struct GameTimer(Timer);
