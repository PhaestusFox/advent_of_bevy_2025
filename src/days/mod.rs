use bevy::prelude::*;

use crate::state::Day;

pub struct DaysPlugin;

impl Plugin for DaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(crate::book_keeping::update_day);
        app.add_plugins(day01::DayPlugin);
        app.add_plugins(day02::DayPlugin);
        app.add_plugins(day03::DayPlugin);
        app.add_plugins(day04::DayPlugin);
        app.add_plugins(day05::DayPlugin);
        app.add_plugins(day06::DayPlugin);
        app.add_plugins(day07::DayPlugin);
    }
}

#[test]
fn gen_day_files() {
    for day in 1..=25 {
        let day_path = format!("assets/days/day{:02}.input", day);
        std::fs::write(day_path, "").unwrap();
    }
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub mod day_s;
#[derive(Event)]
struct Compute<const DAY: usize>;
