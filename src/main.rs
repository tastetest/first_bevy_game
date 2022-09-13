use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::texture::ImageSettings;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod player;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(player::spawn_player)
        .add_startup_system(setup_physics)
        .add_system(player::player_movement)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup_physics(mut commands: Commands) {
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 100.0,))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

#[derive(Copy, Debug, Clone, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}


