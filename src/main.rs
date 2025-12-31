use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(advent_of_bevy_2025::AoCPlugin);

    app.run();
}
