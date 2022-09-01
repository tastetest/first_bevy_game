use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_ecs_ldtk::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugin(LdtkPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_map)
        .add_startup_system(player::spawn_player)
        .insert_resource(LevelSelection::Index(0))
        .add_system(player::player_movement)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {

        commands.spawn_bundle(Camera2dBundle::default());
        asset_server.watch_for_changes().unwrap();

        let ldtk_handle = asset_server.load("./Typical_TopDown_example.ldtk");
        let map_entity = commands.spawn().id();

        commands.entity(map_entity).insert_bundle(LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
        });

}


