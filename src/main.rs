use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod player;
fn main() {
    App::new()
        .add_startup_system(background_tiles)
        .add_startup_system(player::spawn_square)
        .add_system(player::square_move)
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

