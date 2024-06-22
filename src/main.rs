use asteroids::AsteroidsPlugin;
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_sysfail::prelude::*;
use spaceship::SpaceshipPlugin;

use color_eyre::eyre::Result;

mod asteroids;
mod spaceship;

fn main() -> Result<()> {
    color_eyre::install()?;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((SpaceshipPlugin, AsteroidsPlugin))
        .insert_resource(ClearColor(Color::hsl(240., 0.23, 0.09)))
        .add_systems(Startup, spawn_camera)
        .run();
    Ok(())
}

#[sysfail(LogSimply<color_eyre::eyre::Error, Error>)]
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::splat(0.), Vec3::Y),
            tonemapping: Tonemapping::TonyMcMapface,

            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings {
            intensity: 0.2,
            high_pass_frequency: 0.01,
            ..Default::default()
        },
    ));
}
