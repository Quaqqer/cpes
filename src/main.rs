use bevy::prelude::*;

#[derive(Component,  Debug)]
struct Person {

}

fn init(mut commands: Commands) {
    commands.spawn((Person {}));
}


fn print_persons(query: Query<&Person>) {
    for person in query.iter() {
        println!("{:?}", person);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_system(print_persons)
        .run();
}
