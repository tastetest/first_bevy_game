use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// let's create my own plugin i can use
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Jayden Tweedie".to_string()));
    commands.spawn().insert(Person).insert(Name("Peter Parker".to_string()));
    commands.spawn().insert(Person).insert(Name("Bruce Wayne".to_string()));
}
    
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // we are going to update our timer with the time elapsed since last update
    // if that caused that timer to finish, we say hello to everyone
    // this timer will fire names every second  
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run() // these add plugins/add systems are adding the to app's schedule basically
       // so how this works is that systems are functions that contain components 
}






