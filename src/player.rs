use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity_spawn = Vec3::ZERO;
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("./player.png"),
            transform: Transform::from_translation(entity_spawn).with_scale(Vec3::splat(10.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Player);
}

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::W) {
        for mut transform in query.iter_mut() {
            transform.translation.y += 200.2 * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::A) {
        for mut transform in query.iter_mut() {
            transform.translation.x += -200.2 * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::S) {
        for mut transform in query.iter_mut() {
            transform.translation.y += -200.2 * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::D) {
        for mut transform in query.iter_mut() {
            transform.translation.x += 200.2 * time.delta_seconds();
        }
    }
}
