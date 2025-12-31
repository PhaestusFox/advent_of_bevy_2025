use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::{
    Day,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};
const DAY: usize = 2;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);
    }
}
#[derive(Resource)]
struct Products {
    ranges: Vec<RangeInclusive<usize>>,
}

#[derive(Component)]
pub struct Product;
#[derive(Component)]
pub struct ProductId(usize);

impl ProductId {
    pub fn get(&self) -> usize {
        self.0
    }
}

fn parse_input(input: &str, commands: &mut Commands) {
    let mut ranges = Vec::new();
    let root = commands.spawn(DespawnOnExit(Day(DAY as u8))).id();
    for line in input.split(',') {
        let (a, b) = line.split_once('-').unwrap();
        let Ok(a) = a.parse() else {
            error!("Failed to parse range start: {}", a);
            continue;
        };
        let Ok(b) = b.parse() else {
            error!("Failed to parse range end: {}", b);
            continue;
        };
        ranges.push(a..=b);
        commands.spawn((
            ChildOf(root),
            Product,
            children![ProductId(a), ProductId(b)],
        ));
    }
    commands.insert_resource(Products { ranges });
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    parse_input(&input.0, &mut commands);
    commands.trigger(Compute::<DAY>);
}

fn cleanup_day(mut commands: Commands) {
    commands.remove_resource::<Products>();
}

fn solve_part1(_: On<Compute<DAY>>, products: Res<Products>, mut answers: ResMut<Anwsers>) {
    let mut invalid = 0;
    // use a buffer to avoid a new string every loop?
    let mut rep = String::new();
    for range in &products.ranges {
        for id in range.clone() {
            rep.clear();
            use std::fmt::write;
            write(&mut rep, format_args!("{}", id)).unwrap();
            if !rep.len().is_multiple_of(2) {
                continue;
            }
            let half = rep.len() / 2;
            let (first, second) = rep.split_at(half);
            if first == second {
                invalid += id;
            }
        }
    }
    answers.add(DAY, crate::state::Puzzle::Part1, invalid as u64);
}

fn solve_part2(_: On<Compute<DAY>>, products: Res<Products>, mut answers: ResMut<Anwsers>) {
    let mut invalid = 0;
    // use a buffer to avoid a new string every loop?
    let mut rep = String::new();
    for range in &products.ranges {
        for id in range.clone() {
            rep.clear();
            use std::fmt::write;
            write(&mut rep, format_args!("{}", id)).unwrap();
            let max_w = rep.len() / 2;
            'check: for w in (1..=max_w).rev() {
                if !rep.len().is_multiple_of(w) {
                    continue;
                }
                let (prefix, rest) = rep.split_at(w);
                for i in 0..rest.len() / w {
                    let start = i * w;
                    let end = start + w;
                    let chunk = &rest[start..end];
                    if chunk != prefix {
                        continue 'check;
                    }
                }
                invalid += id;
                break;
            }
        }
    }
    answers.add(DAY, crate::state::Puzzle::Part2, invalid as u64);
}

fn spawn_visuals(mut commands: Commands) {}
