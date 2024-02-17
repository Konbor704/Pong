use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

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
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(Startup, (setup, spawn_paddle, spawn_ball, spawn_wall));
    app.add_systems(Update, bevy::window::close_on_esc);

    app.run();
}

fn setup(mut commands: Commands, mut rapier_configuration: ResMut<RapierConfiguration>) {
    commands.spawn(Camera2dBundle::default());

    rapier_configuration.gravity = Vec2::ZERO;
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct WallSides;

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10., 40.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(5., 20.),
        Name::new("Right padle"),
        ActiveEvents::COLLISION_EVENTS,
        Paddle,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10., 40.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(5., 20.),
        Name::new("Left padle"),
        ActiveEvents::COLLISION_EVENTS,
        Paddle,
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(5.),
        Name::new("Ball"),
        ActiveEvents::COLLISION_EVENTS,
        // FIX: velocity jest niezmienna cały czas działa pod skosem
        ExternalImpulse {
            impulse: Vec2::new(1., 1.),
            torque_impulse: 1.
        },
        LockedAxes::ROTATION_LOCKED,
        Ball,
    ));
}

fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10., 300.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(180., 0., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5., 150.),
        Name::new("Wall Right"),
        WallSides,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10., 300.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-180., 0., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5., 150.),
        Name::new("Wall Left"),
        WallSides,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(370., 10.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 150., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(185., 5.),
        Name::new("Wall Up"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(370., 10.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., -150., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(185., 5.),
        Name::new("Wall Down"),
    ));
}
