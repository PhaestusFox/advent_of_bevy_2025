use bevy::{platform::collections::HashMap, prelude::*};

use crate::{Day, PKVKeys, calendar::CalendarState, state::Puzzle};

#[derive(Resource)]
pub struct CurrentDayRaw(pub String);

impl FromWorld for CurrentDayRaw {
    fn from_world(_world: &mut World) -> Self {
        CurrentDayRaw("MainMenu".to_string())
    }
}

impl CurrentDayRaw {
    pub fn load_day(day: u8) -> Option<Self> {
        let path = format!("assets/days/day{:02}.input", day);
        std::fs::read_to_string(path).ok().map(CurrentDayRaw)
    }
}

pub fn update_day(set: On<Day>, mut raw: ResMut<CurrentDayRaw>, mut next: ResMut<NextState<Day>>) {
    next.set(*set);
    if set.0 == 0 {
        return;
    }
    if let Some(data) = CurrentDayRaw::load_day(set.0) {
        *raw = data;
    } else {
        next.set(Day(0));
        error!("Failed to load data for day {}", set.0);
    }
}

#[derive(Resource)]
pub struct Seed(pub u64);
impl FromWorld for Seed {
    fn from_world(world: &mut World) -> Self {
        if let Ok(seed) = world.resource::<bevy_pkv::PkvStore>().get(PKVKeys::Seed) {
            Seed(seed)
        } else {
            let seed = rand::random();
            world
                .resource_mut::<bevy_pkv::PkvStore>()
                .set(PKVKeys::Seed, &seed)
                .unwrap();
            Seed(seed)
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct Anwsers {
    anwsers: HashMap<(usize, Puzzle), u64>,
}

impl Anwsers {
    pub fn add(&mut self, day: usize, puzzle: Puzzle, answer: u64) {
        self.anwsers.insert((day, puzzle), answer);
    }
    pub fn check(&self, day: usize, puzzle: Puzzle, answer: u64) -> Results {
        if let Some(correct) = self.anwsers.get(&(day, puzzle)) {
            if *correct == answer {
                Results::Correct
            } else if *correct < answer {
                Results::Higher
            } else {
                Results::Lower
            }
        } else {
            Results::Missing
        }
    }
}

enum Results {
    Missing,
    Correct,
    Lower,
    Higher,
}

#[derive(Event)]
pub enum Submit {
    Part1(u64),
    Part2(u64),
}

pub(super) fn submit_answers(
    submition: On<Submit>,
    answers: Res<Anwsers>,
    // mut pkv: ResMut<bevy_pkv::PkvStore>,
    day: Res<State<Day>>,
    mut state: ResMut<CalendarState>,
    mut pkv: ResMut<bevy_pkv::PkvStore>,
) {
    println!("Submitting answer for day {}", day.0);
    match &*submition {
        Submit::Part1(answer) => match answers.check(day.0 as usize, Puzzle::Part1, *answer) {
            Results::Missing => error!(
                "No answer stored for day {};\nPester Phox to add this day's answer!",
                day.0
            ),
            Results::Correct => {
                info!("Day {} Part 1 answer is correct!", day.0);
                state.pass(day.0, Puzzle::Part1);
                _ = pkv.set(PKVKeys::CalendarState25, &*state);
            }
            Results::Lower => info!(
                "Day {} Part 1 answer is incorrect; the correct answer is lower.",
                day.0
            ),
            Results::Higher => info!(
                "Day {} Part 1 answer is incorrect; the correct answer is higher.",
                day.0
            ),
        },
        Submit::Part2(answer) => match answers.check(day.0 as usize, Puzzle::Part2, *answer) {
            Results::Missing => error!(
                "No answer stored for day {};\nPester Phox to add this day's answer!",
                day.0
            ),
            Results::Correct => {
                info!("Day {} Part 2 answer is correct!", day.0);
                state.pass(day.0, Puzzle::Part2);
                _ = pkv.set(PKVKeys::CalendarState25, &*state);
            }
            Results::Lower => info!(
                "Day {} Part 2 answer is incorrect; the correct answer is lower.",
                day.0
            ),
            Results::Higher => info!(
                "Day {} Part 2 answer is incorrect; the correct answer is higher.",
                day.0
            ),
        },
    }
}
