use bevy::prelude::*;

pub fn spawn_bullet(
        mut commands: Commands,
        transform: &Transform,
        runstate: &Runstate,
    ) {
    let v = transform.rotation * Vec3::Y * 50.0;
    commands
        .spawn_bundle(SpriteBundle {
           transform: Transform {
               translation: Vec3::new(transform.translation.x, transform.translation.y, 
                                -4.0),
                                rotation: transform.rotation,
                                scale: Vec3::splat(1.0 / 18.0),
           },
           texture: runstate.bullet_texture_handle.clone(),
           ..Default::default()
        })
    .insert(Bullet {
        despawn_timer: Timer::from_seconds(2.0, false),
    })
    .insert(ForState {
        states: vec![AppState::Game],
    })
// do more stuff here    
}

pub fn bullet_timeout_system(
    gamestate: Res<State<AppGameState>>,
    mut bullet_despawn_events: EventWriter<BulletDespawnEvent>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>
    )
    {
        if gamestate.current() == &AppGameState::Game {
            for (entity, mut bullet) in query.iter_mut() {
                bullet.despawn_timer.tick(time.delta());
                if bullet.despawn_timer.finished() {
                    bullet_despawn_events.send(BulletDespawnEvent)
                }
            }
        }
    }
pub fn despawn_bullet_system(
    mut commands: Commands,
    mut event_reader: EventReader<BulletDespawnEvent>,
    ) {
    for event in event_reader.iter() {
        commands.entity(event.0).despawn();
    }
}
