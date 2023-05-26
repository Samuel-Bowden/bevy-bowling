use crate::GameState;
use rand::Rng;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct Config;

impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(ClearColor(Color::hex("#87CEEB").unwrap()))
            .insert_resource(ShootTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    shoot_ball,
                    despawn_fallen,
                    end_condition,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(cleanup_system::<LevelUnload>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Pin;

#[derive(Resource)]
struct ShootTimer(Timer);

#[derive(Component)]
pub struct LevelUnload;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(12., 1., 128.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(6., 0.5, 64.))
        .insert(LevelUnload);

    // Railings
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(1., 2., 128.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(-4.5, 1., 0.),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 1., 64.))
        .insert(LevelUnload);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(1., 1., 128.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(4.5, 1., 0.),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5, 64.))
        .insert(LevelUnload);

    // Pins
    for i in 0..4 {
        let mut c = -i;
        while c <= i {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(shape::Box::new(0.5, 2., 0.5).into()),
                    material: materials.add(Color::WHITE.into()),
                    transform: Transform::from_xyz((c as f32) / 1.5, 2., -25. - i as f32),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(0.25, 1., 0.25))
                .insert(ColliderMassProperties::Density(0.1))
                .insert(Ccd::enabled())
                .insert(Pin);

            c += 2;
        }
    }

    // Light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 200000.,
                range: 2000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(40., 30., -10.),
            ..default()
        })
        .insert(LevelUnload);

    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0., 8., -50.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(LevelUnload);
}

fn shoot_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        let colours = [Color::PURPLE, Color::BLUE, Color::PINK, Color::RED];
        let colour = colours[rng.gen_range(0..3)];
        let radius = rng.gen_range(0.25..0.5);
        let transform_x = rng.gen_range(-2.0..2.);
        let curve = rng.gen_range(-5.0..5.);
        let speed = rng.gen_range(-100.0..-40.);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius,
                        subdivisions: 10,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(colour.into()),
                transform: Transform::from_xyz(transform_x, 2.5, 58.),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(ColliderMassProperties::Density(10.0))
            .insert(Velocity {
                linvel: Vec3::new(curve, 0., speed),
                ..Default::default()
            })
            .insert(Collider::ball(radius))
            .insert(Ccd::enabled())
            .insert(Ball)
            .insert(LevelUnload);
    }
}

fn despawn_fallen(
    q: Query<(Entity, &Transform), Or<(With<Ball>, With<Pin>)>>,
    mut commands: Commands,
) {
    for (entity, transform) in q.iter() {
        if transform.translation.y <= -5. {
            commands.entity(entity).despawn();
        }
    }
}

fn end_condition(q: Query<&Pin>, mut game_state: ResMut<NextState<GameState>>) {
    if q.iter().len() == 0 {
        game_state.set(GameState::MainMenu);
    }
}

fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
