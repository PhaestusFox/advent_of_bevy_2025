use bevy::prelude::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Puzzle {
    #[default]
    Part1,
    Part2,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Component, Default, Copy, Event)]
pub struct Day(pub u8);

impl Day {
    pub fn day(&self) -> u8 {
        self.0
    }
}
