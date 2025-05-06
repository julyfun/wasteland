use bevy::prelude::*;

#[derive(Component)]
struct Person;

// name
#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, greet_people)
            .add_systems(Update, rename);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

/// iterate over every Name component for entities that also have a Person component".
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}!", name.0);
        }
    }
}

fn rename(mut query: Query<&mut Name, With<Person>>) {
    for mut name in query.iter_mut() {
        if name.0 == "Alice" {
            name.0 = "Alex".to_string();
            break;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
