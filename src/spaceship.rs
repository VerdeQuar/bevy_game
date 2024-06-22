use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_sysfail::prelude::*;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship)
            .add_systems(Update, move_spaceship);
    }
}

#[derive(Component, Debug)]
struct Acceleration(Vec3);

#[derive(Component, Debug)]
struct Velocity(Vec3);

#[derive(Component, Debug)]
struct Spaceship;

#[sysfail(LogSimply<color_eyre::eyre::Error, Error>)]
fn spawn_spaceship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Spaceship,
        Acceleration(Vec3::splat(0.)),
        Velocity(Vec3::splat(0.)),
        ColorMesh2dBundle {
            transform: Transform::from_xyz(0., 0., 1.),
            material: materials.add(Color::hsl(0., 0., 1.)),
            mesh: meshes.add(RegularPolygon::new(50., 3)).into(),
            ..Default::default()
        },
    ));
}

#[sysfail(LogSimply<color_eyre::eyre::Error, Error>)]
fn move_spaceship(
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut spaceship: Query<(&mut Acceleration, &mut Velocity, &mut Transform), With<Spaceship>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let (mut acceleration, mut velocity, mut transform) = spaceship.get_single_mut()?;

    acceleration.0 = Vec3::ZERO;

    for key in keys.get_pressed() {
        match key {
            KeyCode::ArrowRight => transform.rotate_z(-0.05),
            KeyCode::ArrowLeft => transform.rotate_z(0.05),
            KeyCode::ArrowUp => {
                acceleration.0 += (transform.rotation * Vec3::Y).normalize_or_zero()
            }
            KeyCode::ArrowDown => {
                acceleration.0 -= (transform.rotation * Vec3::Y).normalize_or_zero() * 0.5
            }
            _ => {}
        }
    }

    velocity.0 =
        (velocity.0 + (acceleration.0 * 100. * time.delta_seconds())).clamp_length_max(40.);

    velocity.0 *= 0.9;
    transform.scale = Vec3::ONE
        + (Vec3::Y * (velocity.0.dot(transform.rotation * Vec3::Y)).signum() * velocity.0.length()
            / 125.);
    transform.translation += velocity.0;

    let window = windows.get_single()?;
    let width = window.width();
    let height = window.height();

    if transform.translation.x - 50. > width / 2.0 {
        transform.translation.x = (-width / 2.0) - 50.;
    }

    if transform.translation.x + 50. < -width / 2.0 {
        transform.translation.x = (width / 2.0) + 50.;
    }

    if transform.translation.y - 50. > height / 2.0 {
        transform.translation.y = (-height / 2.0) - 50.;
    }

    if transform.translation.y + 50. < -height / 2.0 {
        transform.translation.y = (height / 2.0) + 50.;
    }
}
