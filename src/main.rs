use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(spawn_square)
        .add_system(square_move)
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct Square;

fn spawn_square(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity_spawn = Vec3::ZERO;
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("./icon.png"),
        transform: Transform::from_translation(entity_spawn),
        ..default()
    })
    .insert(Square);

}

fn square_move(mut query: Query<&mut Transform, With<Square>>) {
        for mut transform in query.iter_mut() {
            transform.translation += 1.2;
        }
}
