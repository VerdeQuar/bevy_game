use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_mod_sysfail::prelude::*;
use rand::{thread_rng, Rng};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_asteroids)
            .add_systems(Update, animate_asteroids);
    }
}

#[derive(Component, Debug)]
struct Asteroid {
    rotation_speed: f32,
}

#[sysfail(LogSimply<color_eyre::eyre::Error, Error>)]
fn spawn_asteroids(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single()?;

    for i in 1..200 {
        let mut rng = thread_rng();
        commands.spawn((
            Asteroid {
                rotation_speed: 1. / rng.gen_range(-10..10) as f32,
            },
            MaterialMesh2dBundle {
                transform: Transform::from_xyz(
                    rng.gen_range(-window.width()..window.width()),
                    rng.gen_range(-window.height()..window.height()),
                    -i as f32,
                ),
                material: materials.add(Color::hsl(rng.gen_range(0..360) as f32, 0.7, 0.8)),
                mesh: meshes
                    .add(RegularPolygon::new(
                        rng.gen_range(10..70) as f32,
                        rng.gen_range(5..9),
                    ))
                    .into(),
                ..Default::default()
            },
        ));
    }
}

#[sysfail(LogSimply<color_eyre::eyre::Error, Error>)]
fn animate_asteroids(
    time: Res<Time>,
    mut asteroids: Query<(&Asteroid, &mut Transform), With<Asteroid>>,
) {
    for (asteroid, mut transform) in asteroids.iter_mut() {
        transform.rotate_z(asteroid.rotation_speed * time.delta_seconds());
        transform.scale = (transform.scale - 0.01 * time.delta_seconds()).clamp_length_min(0.);
    }
}
