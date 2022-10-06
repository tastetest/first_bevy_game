use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData, NinePatchPlugin};

mod player;
mod ui;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(NinePatchPlugin::<()>::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .add_startup_system(player::spawn_player)
        .add_startup_system(setup_physics)
        .add_startup_system(setup_ldtk)
        .add_startup_system(ui::setup)
        .add_system(player::player_movement)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup_physics(mut commands: Commands) {
    commands
        .spawn()
        .insert(Collider::cuboid(300.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(ActiveCollisionTypes::default())
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load("my_project.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
