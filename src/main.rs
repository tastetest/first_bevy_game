use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod player;

fn main() {
    App::new()
        .add_startup_system(player::spawn_player)
        .add_system(player::player_movement)
        .add_system(enemy_spawn)
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}


fn background_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("./background_dirt.png"),
        ..default()
    });
}

fn enemy_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_init_coords = Vec3::ZERO;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("./enemy.png"),
        transform: Transform::from_translation(enemy_init_coords)
            .with_scale(Vec3::splat(2.0)),
        ..default()

    });
}
