use bevy::{platform::collections::HashSet, prelude::*};

use crate::{
    Day,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};
const DAY: usize = 5;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);
    }
}

#[derive(Debug, Clone, Copy)]
struct Fresh {
    start: usize,
    end: usize,
}

#[derive(Resource, Debug)]
pub struct FreshList {
    list: Vec<Fresh>,
}

impl FreshList {
    fn is_fresh(&self, ing: usize) -> bool {
        for fresh in &self.list {
            if ing < fresh.start || ing > fresh.end {
                continue;
            }
            return true;
        }
        false
    }
}

#[derive(Component)]
pub struct Ingredient(pub usize);

fn parse_input(input: &str, commands: &mut Commands) {
    let mut fresh = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let Ok(start) = start.parse() else {
            error!("Failed to parse fresh start: {}", start);
            continue;
        };
        let Ok(end) = end.parse() else {
            error!("Failed to parse fresh end: {}", end);
            continue;
        };
        fresh.push(Fresh { start, end });
    }
    fresh.sort_by(|a, b| a.start.cmp(&b.start));
    for line in input.lines().skip(fresh.len() + 1) {
        let Ok(ing) = line.parse() else {
            error!("Failed to parse ingredient: {}", line);
            continue;
        };
        commands.spawn(Ingredient(ing));
    }

    commands.insert_resource(FreshList { list: fresh });
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    parse_input(&input.0, &mut commands);
    commands.trigger(Compute::<DAY>);
}

fn cleanup_day(mut commands: Commands) {}

fn solve_part1(
    _: On<Compute<DAY>>,
    fresh: Res<FreshList>,
    query: Query<&Ingredient>,
    mut answers: ResMut<Anwsers>,
) {
    let mut fresh_ing = 0;
    // info!("{:?}", fresh);
    for Ingredient(ing) in &query {
        if fresh.is_fresh(*ing) {
            fresh_ing += 1;
        }
    }

    answers.add(DAY, crate::state::Puzzle::Part1, fresh_ing as u64);
    // info!("Part 1: {fresh_ing}");
}

fn solve_part2(_: On<Compute<DAY>>, fresh: Res<FreshList>, mut answers: ResMut<Anwsers>) {
    let mut dedup_fresh = vec![fresh.list[0], *fresh.list.last().unwrap()];
    'out: for fresh in &fresh.list[1..] {
        for dedup in &mut dedup_fresh {
            if fresh.start <= dedup.end && fresh.end >= dedup.start {
                dedup.start = dedup.start.min(fresh.start);
                dedup.end = dedup.end.max(fresh.end);
                continue 'out;
            }
        }
        dedup_fresh.push(*fresh);
    }

    let mut total_fresh = 0;
    for fresh in &dedup_fresh {
        total_fresh += fresh.end - fresh.start + 1;
    }

    for i in 0..dedup_fresh.len() - 1 {
        let r = &dedup_fresh[i];
        for other in &dedup_fresh[i + 1..] {
            if r.end > other.start && r.end < other.end {
                panic!("Ranges {:?} and {:?} overlap", r, other);
            }
            if r.start < other.end && r.start > other.start {
                panic!("Ranges {:?} and {:?} overlap", r, other);
            }
        }
    }

    // info!(
    //     "Fresh: {}\n dedup: {:?}",
    //     fresh.list.len(),
    //     dedup_fresh.len()
    // );
    answers.add(DAY, crate::state::Puzzle::Part2, total_fresh as u64);
    // info!("Part 2: {}", total_fresh);
}

fn spawn_visuals(mut commands: Commands) {}
// 329989062881448 low
// 358155203664116
