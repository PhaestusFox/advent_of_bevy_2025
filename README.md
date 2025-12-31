A bevy of skin on top of Advent of Code.

# To use
Add advent_of_bevy_2025 to the project
Add Bevy 0.17 to the project
In fn main(), add AoBPlugin to an empty Bevy app
```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(advent_of_bevy_2025::AoCPlugin);

    app.run();
}
```
run add
Put AoC puzzle input in Day*.input file
attempt to solve puzzle using provided Types in advent_of_bevy::day*
When the game is running, click a day to load the puzzle
When the puzzle is solved, trigger Submit::Puzzle*(answer)
will update calendar if correct
