use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    Day,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};
const DAY: usize = 7;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Map {
    size: IVec2,
    #[deref]
    map: HashMap<IVec2, Entity>,
}

impl Map {
    pub fn x(&self) -> i32 {
        self.size.x
    }
    pub fn y(&self) -> i32 {
        self.size.y
    }
}

#[derive(Component)]
pub struct Start;

#[derive(Component)]
pub struct Spliter;

fn parse_input(input: &str, commands: &mut Commands) {
    let width = input.split('\n').next().unwrap().trim().len();
    let height = input.lines().count();
    let mut map = Map {
        map: HashMap::new(),
        size: IVec2::new(width as i32, height as i32),
    };
    commands
        .spawn((
            Node {
                height: Val::Percent(95.0),
                aspect_ratio: Some(1.),
                display: Display::Grid,
                margin: UiRect::all(Val::Auto),
                grid_auto_flow: GridAutoFlow::Column,
                grid_template_columns: vec![RepeatedGridTrack::fr(width as u16, 1.0)],
                grid_template_rows: vec![RepeatedGridTrack::fr(height as u16, 1.0)],
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(0.0, 0.0, 1.0)),
            DespawnOnExit(Day(DAY as u8)),
        ))
        .with_children(|parent| {
            for (row, line) in input.lines().enumerate() {
                for (col, ch) in line.char_indices() {
                    let color = match ch {
                        'S' => Color::linear_rgb(0.0, 1.0, 0.0),
                        '^' => Color::linear_rgb(1.0, 0.0, 0.0),
                        _ => Color::WHITE,
                    };
                    let mut c = parent.spawn((
                        Node {
                            aspect_ratio: Some(1.0),
                            grid_row: GridPlacement::start(row as i16 + 1),
                            grid_column: GridPlacement::start(col as i16 + 1),
                            ..Default::default()
                        },
                        BackgroundColor(color),
                    ));
                    match ch {
                        'S' => {
                            c.insert(Start);
                        }
                        '^' => {
                            c.insert(Spliter);
                        }
                        _ => {}
                    }
                    map.insert(IVec2::new(col as i32, row as i32), c.id());
                }
            }
        });
    commands.insert_resource(map);
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    parse_input(&input.0, &mut commands);
    commands.trigger(Compute::<DAY>);
}

fn cleanup_day(mut commands: Commands) {
    commands.remove_resource::<Map>();
}

const ACTIVE_COLOR: Color = Color::linear_rgb(0.0, 0.0, 1.0);
const SPLIT_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
const TRIGGERED: Color = Color::linear_rgb(1.0, 0.0, 1.0);

fn solve_part1(
    _: On<Compute<DAY>>,
    start: Single<&Node, With<Start>>,
    map: Res<Map>,
    mut nodes: Query<&mut BackgroundColor>,
    splitter: Query<Entity, With<Spliter>>,
    mut answers: ResMut<Anwsers>,
) {
    let (x, mut y) = if let Some(x) = start.grid_column.get_start()
        && let Some(y) = start.grid_row.get_start()
    {
        (x as i32 - 1, y as i32)
    } else {
        info!("Part 1: Start Position not found");
        return;
    };
    let first = IVec2::new(x, y);
    let Some(first) = map.get(&first) else {
        info!("Next Position not found in map");
        return;
    };
    let mut color = nodes.get_mut(*first).unwrap();
    if color.0 == Color::WHITE {
        color.0 = ACTIVE_COLOR;
    } else {
        error!("Start position is not white");
    }
    while y < map.y() {
        for x in 0..map.x() {
            let pos = IVec2::new(x, y);
            let Some(node) = map.get(&pos) else {
                info!("{} not found in map", pos);
                continue;
            };
            let Ok(color) = nodes.get(*node) else {
                info!("Node({:?}) not found in query", pos);
                continue;
            };
            if color.0 == ACTIVE_COLOR {
                let down = pos + IVec2::Y;
                let Some(down_e) = map.get(&down) else {
                    info!("Next Position not found in map");
                    continue;
                };
                let Ok(mut next) = nodes.get_mut(*down_e) else {
                    info!("Node({:?}) not found in query", down);
                    continue;
                };
                if next.0 == Color::WHITE {
                    next.0 = ACTIVE_COLOR;
                } else if next.0 == SPLIT_COLOR {
                    next.0 = TRIGGERED;
                    if down.x > 0 {
                        let left = down - IVec2::X;
                        let Some(left_e) = map.get(&left) else {
                            info!("Next Position not found in map");
                            continue;
                        };
                        let Ok(mut left_n) = nodes.get_mut(*left_e) else {
                            info!("Node({:?}) not found in query", left);
                            continue;
                        };
                        if left_n.0 == Color::WHITE {
                            left_n.0 = ACTIVE_COLOR;
                        }
                    }
                    if down.y > 0 {
                        let right = down + IVec2::X;
                        let Some(right_e) = map.get(&right) else {
                            info!("Next Position not found in map");
                            continue;
                        };
                        let Ok(mut right_n) = nodes.get_mut(*right_e) else {
                            info!("Node({:?}) not found in query", right);
                            continue;
                        };
                        if right_n.0 == Color::WHITE {
                            right_n.0 = ACTIVE_COLOR;
                        }
                    }
                }
            }
        }
        y += 1;
    }
    let mut count = 0;
    for splitter in &splitter {
        let Ok(color) = nodes.get(splitter) else {
            info!("Splitter Node not found in query");
            continue;
        };
        if color.0 == TRIGGERED {
            count += 1;
        }
    }

    answers.add(DAY, crate::state::Puzzle::Part1, count);
    // info!("Part 1: {}", count);
}

fn solve_part2(
    _: On<Compute<DAY>>,
    start: Single<&Node, With<Start>>,
    map: Res<Map>,
    splitter: Query<Entity, With<Spliter>>,
    mut answers: ResMut<Anwsers>,
) {
    let (x, y) = if let Some(x) = start.grid_column.get_start()
        && let Some(y) = start.grid_row.get_start()
    {
        (x as i32 - 1, y as i32 - 1)
    } else {
        info!("Part 1: Start Position not found");
        return;
    };
    let mut paths = HashMap::new();
    let mut next_paths = HashMap::new();
    paths.insert(IVec2::new(x, y), 1);
    for y in 0..map.y() {
        next_paths.clear();
        for (path, count) in paths.iter_mut() {
            let Some(down_e) = map.get(path) else {
                info!("Next({:?}) Position not found in map", path);
                continue;
            };
            let path = path + IVec2::Y;
            if splitter.contains(*down_e) {
                next_paths
                    .entry(path - IVec2::X)
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
                next_paths
                    .entry(path + IVec2::X)
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            } else {
                next_paths
                    .entry(path)
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            }
        }
        println!("Paths at y={}: {:?}", y, paths.len());
        if y < 8 {
            println!("{:#?}", paths);
        }
        std::mem::swap(&mut paths, &mut next_paths);
    }
    answers.add(
        DAY,
        crate::state::Puzzle::Part2,
        paths.values().sum::<usize>() as u64,
    );
    // info!("Part 2: {}", paths.values().sum::<usize>());
}

fn spawn_visuals(mut commands: Commands) {}
