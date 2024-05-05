use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::WindowResolution,
};
use rand::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
    app.insert_resource(ScoreBoard {
        score1: 0,
        score2: 0,
    });
    app.add_event::<CollisionEvent>();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(500., 500.),
                    resizable: false,
                    title: "Pong".to_string(),
                    ..default()
                }),
                ..default()
            }),
    );

    app.add_systems(
        Startup,
        (
            setup,
            spawn_paddle,
            spawn_ball,
            spawn_wall,
            spawn_scoreboard,
        ),
    );
    app.add_systems(
        Update,
        (bevy::window::close_on_esc, move_paddle1, move_paddle2),
    );
    app.add_systems(FixedUpdate, (applay_vleocity, check_collision).chain());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle1;

#[derive(Component)]
struct Paddle2;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct WallSides;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

const WALL_LEFT_LOC: f32 = -180.;
const WALL_RIGHT_LOC: f32 = 180.;
const WALL_TOP_LOC: f32 = 150.;
const WALL_BOTTOM_LOC: f32 = -150.;

const WALL_LEFT_SIZE: Vec2 = Vec2::new(10., 300.);
const WALL_RIGHT_SIZE: Vec2 = Vec2::new(10., 300.);
const WALL_TOP_SIZE: Vec2 = Vec2::new(370., 10.);
const WALL_BOTTOM_SIZE: Vec2 = Vec2::new(370., 10.);

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(-180., 0.),
            WallLocation::Right => Vec2::new(180., 0.),
            WallLocation::Bottom => Vec2::new(0., -150.),
            WallLocation::Top => Vec2::new(0., 150.),
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(10., 300.),
            WallLocation::Right => Vec2::new(10., 300.),
            WallLocation::Top => Vec2::new(370., 10.),
            WallLocation::Bottom => Vec2::new(370., 10.),
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

const PADDLE_SIZE: Vec3 = Vec3::new(10., 40., 0.);

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(150., 0.0, 0.0),
                scale: PADDLE_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("Right padle"),
        Paddle2,
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-150., 0.0, 0.0),
                scale: PADDLE_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("Right Left"),
        Paddle1,
        Collider,
    ));
}

const PADDLE_PADDING: f32 = 13.;
const TOP_BOUND: f32 = 110.;
const BOTTOM_BOUND: f32 = -110.;

fn move_paddle1(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle1>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.y + direction * 300. * time.delta_seconds();

    // let top_bound = WALL_TOP_LOC + WALL_TOP_SIZE.y / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;
    // let bottom_bound =
    //     WALL_BOTTOM_LOC - WALL_BOTTOM_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.y = new_paddle_position.clamp(BOTTOM_BOUND, TOP_BOUND);
}
fn move_paddle2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle2>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyJ) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyK) {
        direction += 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.y + direction * 300. * time.delta_seconds();

    // let top_bound = WALL_TOP_LOC + WALL_TOP_SIZE.y / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;
    // let bottom_bound =
    //     WALL_BOTTOM_LOC - WALL_BOTTOM_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.y = new_paddle_position.clamp(BOTTOM_BOUND, TOP_BOUND);
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::new(5., 5., 0.)),
            ..default()
        },
        Ball,
        Velocity(Vec2::new(0.5, 0.5) * 180.),
    ));
}

fn spawn_wall(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}

#[derive(Resource)]
struct ScoreBoard {
    score1: usize,
    score2: usize,
}

#[derive(Component)]
struct ScoreBoardUi1;

#[derive(Component)]
struct ScoreBoardUi2;

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        ScoreBoardUi1,
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: 40.,
            color: Color::WHITE,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.),
            left: Val::Px(240.),
            ..default()
        }),
    ));

    commands.spawn((
        ScoreBoardUi2,
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: 40.,
            color: Color::WHITE,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.),
            left: Val::Px(260.),
            ..default()
        }),
    ));
}

fn applay_vleocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&WallSides>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    for (collider_entity, transform, maybe_wallsides) in &collider_query {
        let collision = collide_with_side(
            BoundingCircle::new(ball_transform.translation.truncate(), 10. / 2.),
            Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            let mut rng = thread_rng();
            let diff: f32 = rng.gen();

            if reflect_x {
                ball_velocity.x = -ball_velocity.x + diff;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y + diff;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
