use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::{
    Day,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};
const DAY: usize = 4;

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
struct PaperRoles {
    map: HashSet<IVec2>,
    size: IVec2,
}

#[derive(Component)]
pub struct Role;
#[derive(Component, Deref)]
pub struct Position(IVec2);

fn parse_input(input: &str, commands: &mut Commands) {
    let mut roles = HashSet::new();
    let mut size = IVec2::ZERO;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '@' {
                roles.insert(IVec2::new(x as i32, y as i32));
            }
        }
        size.y = y as i32 + 1;
        size.x = line.len() as i32;
    }
    commands
        .spawn((
            Node {
                aspect_ratio: Some(1.),
                height: Val::Percent(95.),
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Px(25.)),
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::fr(size.x as u16, 1.)],
                grid_template_rows: vec![RepeatedGridTrack::fr(size.y as u16, 1.)],
                ..Default::default()
            },
            BackgroundColor(Color::BLACK.lighter(0.2)),
            BorderRadius::all(Val::Px(25.)),
            DespawnOnExit(Day(DAY as u8)),
        ))
        .with_children(|p| {
            for cell in roles.iter() {
                p.spawn((
                    Node {
                        grid_column: GridPlacement::start(cell.x as i16 + 1),
                        grid_row: GridPlacement::start(cell.y as i16 + 1),
                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    BorderRadius::all(Val::Percent(50.)),
                    Role,
                    Position(*cell),
                ));
            }
        });
    commands.insert_resource(PaperRoles { map: roles, size });
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    parse_input(&input.0, &mut commands);
    commands.trigger(Compute::<DAY>);
}

fn cleanup_day(mut commands: Commands) {
    commands.remove_resource::<PaperRoles>();
}

fn solve_part1(
    _: On<Compute<DAY>>,
    map: Res<PaperRoles>,
    mut roles: Query<(&Node, &mut BackgroundColor)>,
    mut answers: ResMut<Anwsers>,
) {
    let mut valid = 0;
    for (node, mut color) in roles.iter_mut() {
        if let Some(x) = node.grid_column.get_start()
            && let Some(y) = node.grid_row.get_start()
        {
            if !check(IVec2::new(x as i32 - 1, y as i32 - 1), &map.map) {
                color.0 = Color::linear_rgb(0., 1., 0.);
                valid += 1;
            }
        } else {
            continue;
        }
    }
    answers.add(DAY, crate::state::Puzzle::Part1, valid as u64);
}

fn solve_part2(
    _: On<Compute<DAY>>,
    map: Res<PaperRoles>,
    mut roles: Query<(Entity, &Node, &mut BackgroundColor)>,
    mut answers: ResMut<Anwsers>,
) {
    let mut to_remove = HashMap::new();
    let mut total_removed = 0;
    let mut hue = 0.;
    let mut left = roles.iter().map(|(e, _, _)| e).collect::<Vec<_>>();
    let mut map = map.map.clone();
    loop {
        for e in left.iter() {
            let (_, node, mut color) = roles.get_mut(*e).unwrap();

            if let Some(x) = node.grid_column.get_start()
                && let Some(y) = node.grid_row.get_start()
            {
                if !check(IVec2::new(x as i32 - 1, y as i32 - 1), &map) {
                    color.0 = Color::hsl(hue, 1., 0.5);
                    to_remove.insert(*e, IVec2::new(x as i32 - 1, y as i32 - 1));
                    total_removed += 1;
                }
            } else {
                continue;
            }
        }
        hue += 137.508; // golden angle in degrees
        if to_remove.is_empty() {
            break;
        }
        left.retain(|x| !to_remove.contains_key(x));
        for (_, pos) in to_remove.drain() {
            map.remove(&pos);
        }
    }
    answers.add(DAY, crate::state::Puzzle::Part2, total_removed as u64);
}

fn spawn_visuals(mut commands: Commands) {}

fn check(pos: IVec2, map: &HashSet<IVec2>) -> bool {
    let mut found = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if map.contains(&(pos + IVec2::new(dx, dy))) {
                found += 1;
            }
        }
    }
    // println!("Total found at ({},{}): {}", pos.x, pos.y, found);
    found > 4
}
