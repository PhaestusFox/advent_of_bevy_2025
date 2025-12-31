use bevy::prelude::*;

use crate::{
    Day,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};
const DAY: usize = 6;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);
    }
}

#[derive(Component)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

#[derive(Component)]
struct Cephalopod;

impl Problem {
    fn push(&mut self, value: usize) {
        self.numbers.push(value);
    }
    fn set_add(&mut self) {
        self.operation = Operation::Add;
    }
    fn set_multiply(&mut self) {
        self.operation = Operation::Multiply;
    }
    fn compute(&self) -> usize {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }
}

enum Operation {
    Add,
    Multiply,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
        }
    }
}

fn parse_input(input: &str, commands: &mut Commands) {
    let mut problems: Vec<(Problem, DespawnOnExit<Day>)> = Vec::new();
    for line in input.lines() {
        for (i, token) in line.split_whitespace().enumerate() {
            if problems.len() <= i {
                problems.push((
                    Problem {
                        numbers: Vec::new(),
                        operation: Operation::Add,
                    },
                    DespawnOnExit(Day(DAY as u8)),
                ));
            }
            if token == "+" {
                problems[i].0.set_add();
                continue;
            }
            if token == "*" {
                problems[i].0.set_multiply();
                continue;
            }
            if let Ok(value) = token.parse() {
                problems[i].0.push(value);
                continue;
            }
        }
    }
    commands.spawn_batch(problems);

    let mut lines = input.lines().map(|l| l.chars()).collect::<Vec<_>>();
    let mut column = Vec::new();
    let mut block: Vec<Vec<char>> = vec![Vec::new(); lines.len()];
    loop {
        let Some(next) = lines
            .iter_mut()
            .map(|c| c.next())
            .collect::<Option<Vec<_>>>()
        else {
            break;
        };
        if next.iter().all(|c| c == &' ') {
            let mut new = vec![Vec::new(); lines.len()];
            std::mem::swap(&mut block, &mut new);
            column.push(new);
            continue;
        }
        for (c, b) in next.into_iter().zip(block.iter_mut()) {
            b.push(c);
        }
    }
    column.push(block);
    let mut problems = Vec::new();
    for mut col in column {
        let op = if col.pop().unwrap().contains(&'+') {
            Operation::Add
        } else {
            Operation::Multiply
        };
        let mut problem = Problem {
            numbers: Vec::new(),
            operation: op,
        };
        let mut value = String::new();
        for column in 0..col[0].len() {
            value.clear();
            for row in col.iter() {
                value.push(row[column]);
            }
            problem.push(value.trim().parse().unwrap());
        }
        problems.push((problem, Cephalopod, DespawnOnExit(Day(DAY as u8))));
    }
    commands.spawn_batch(problems);
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    parse_input(&input.0, &mut commands);
    commands.trigger(Compute::<DAY>);
}

fn cleanup_day(mut commands: Commands) {}

fn solve_part1(
    _: On<Compute<DAY>>,
    problems: Query<&Problem, Without<Cephalopod>>,
    mut answers: ResMut<Anwsers>,
) {
    let mut total = 0;
    for problem in &problems {
        total += problem.compute();
    }
    answers.add(DAY, crate::state::Puzzle::Part1, total as u64);
    // info!("Part 1: {}", total);
}

fn solve_part2(
    _: On<Compute<DAY>>,
    problems: Query<&Problem, With<Cephalopod>>,
    mut answers: ResMut<Anwsers>,
) {
    let mut total = 0;
    for problem in &problems {
        // println!(
        //     "{:?} {} = {}",
        //     problem.numbers,
        //     problem.operation,
        //     problem.compute()
        // );
        total += problem.compute();
    }
    answers.add(DAY, crate::state::Puzzle::Part2, total as u64);
    // info!("Part 2: {}", total);
}
fn spawn_visuals(mut commands: Commands) {}

// 11159825692437 low
