use bevy::prelude::*;

use crate::{Day, book_keeping::CurrentDayRaw, days::Solve};
const DAY: usize = 0;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);
    }
}

fn parse_input(input: &str, commands: &mut Commands) {}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    parse_input(&input.0, &mut commands);
    commands.trigger(Solve::<DAY>);
}

fn cleanup_day(mut commands: Commands) {}

fn solve_part1(_: On<Solve<DAY>>) {
    info!("Part 1: Not Solved");
}

fn solve_part2(_: On<Solve<DAY>>) {
    info!("Part 2: Not Solved");
}

fn spawn_visuals(mut commands: Commands) {}
