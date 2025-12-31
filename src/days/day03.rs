use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::{
    Day,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};
const DAY: usize = 3;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);

        app.add_systems(
            Update,
            (draw_number_line, init_12_slot, scan).run_if(in_state(Day(DAY as u8))),
        );
    }
}

#[derive(Component, Clone, Copy)]
#[component(on_add = Self::on_add)]
#[require(Node = Node {
    width: Val::Percent(100.),
    min_height: Val::Vw(0.6),
    flex_direction: FlexDirection::Row,
    padding: UiRect::all(Val::Percent(1.)),
    justify_content: JustifyContent::SpaceBetween,
    ..Node::DEFAULT
},
BackgroundColor = BackgroundColor(Color::linear_rgb(0., 1., 0.)),
BorderRadius = BorderRadius::all(Val::Percent(5.))
)]
pub enum Battery {
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
}

impl Battery {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let battery = *world
            .get::<Battery>(ctx.entity)
            .expect("Component Just Added");
        let mut color = world
            .get_mut::<BackgroundColor>(ctx.entity)
            .expect("Node is required");
        color.0 = battery.color();
        world.commands().entity(ctx.entity).insert(children![
            (
                Node {
                    border: UiRect::all(Val::Px(1.)),
                    height: Val::Percent(5.0),
                    min_height: Val::Vw(0.5),
                    aspect_ratio: Some(1.0),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                BorderRadius::all(Val::Percent(50.)),
                BorderColor::all(Color::BLACK.lighter(0.33)),
                BackgroundColor(Color::linear_rgb(1., 0., 0.)),
            ),
            (
                Node {
                    border: UiRect::all(Val::Px(1.)),
                    height: Val::Percent(5.0),
                    min_height: Val::Vw(0.5),
                    aspect_ratio: Some(1.0),
                    right: Val::Px(0.),
                    ..Default::default()
                },
                BorderRadius::all(Val::Percent(50.)),
                BorderColor::all(Color::BLACK.lighter(0.33)),
                BackgroundColor(Color::linear_rgb(0., 0., 1.)),
            )
        ]);
    }

    pub fn level(&self) -> usize {
        match self {
            Battery::P1 => 1,
            Battery::P2 => 2,
            Battery::P3 => 3,
            Battery::P4 => 4,
            Battery::P5 => 5,
            Battery::P6 => 6,
            Battery::P7 => 7,
            Battery::P8 => 8,
            Battery::P9 => 9,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Battery::P1 => Color::linear_rgb(0.1, 0.01, 0.01),
            Battery::P2 => Color::linear_rgb(0.5, 0.01, 0.01),
            Battery::P3 => Color::linear_rgb(1.0, 0.02, 0.01),
            Battery::P4 => Color::linear_rgb(1.0, 0.4, 0.1),
            Battery::P5 => Color::linear_rgb(1.0, 1.0, 0.1),
            Battery::P6 => Color::linear_rgb(0.3, 0.7, 0.1),
            Battery::P7 => Color::linear_rgb(0.05, 0.6, 0.1),
            Battery::P8 => Color::linear_rgb(0.01, 0.8, 0.01),
            Battery::P9 => Color::linear_rgb(0.01, 1.0, 0.01),
        }
    }
}

#[derive(Component)]
#[component(on_add = Self::on_add)]
#[require(Node)]
pub struct Bank;
impl Bank {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let mut node = world.get_mut::<Node>(ctx.entity).expect("Node is Required");
        node.width = Val::Percent(95.);
    }
}

fn parse_input(input: &str) -> Vec<Vec<Battery>> {
    let mut banks = Vec::new();
    for line in input.lines() {
        let mut bank = Vec::new();
        for ch in line.chars() {
            let battery = match ch {
                '1' => Battery::P1,
                '2' => Battery::P2,
                '3' => Battery::P3,
                '4' => Battery::P4,
                '5' => Battery::P5,
                '6' => Battery::P6,
                '7' => Battery::P7,
                '8' => Battery::P8,
                '9' => Battery::P9,
                _ => continue,
            };
            bank.push(battery);
        }
        banks.push(bank);
    }
    banks
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    let bank_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(10.)),
        flex_grow: 1.0,
        flex_wrap: FlexWrap::Wrap,
        display: Display::Grid,
        grid_auto_flow: GridAutoFlow::Row,
        min_height: Val::Vw(0.5),
        grid_template_columns: vec![RepeatedGridTrack::percent(25, 4.)],
        grid_template_rows: vec![RepeatedGridTrack::percent(4, 25.)],
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        justify_items: JustifyItems::Stretch,
        align_items: AlignItems::Stretch,
        ..Default::default()
    };
    let banks = parse_input(&input.0).into_iter().map(|bank| {
        (
            BackgroundColor(Color::Srgba(bevy::color::palettes::css::TAN).lighter(0.25)),
            BorderRadius::all(Val::Px(15.)),
            Children::spawn(bank),
            Bank,
            bank_node.clone(),
        )
    });
    commands
        .spawn((
            Node {
                width: Val::Percent(95.0),
                height: Val::Percent(98.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Percent(0.5)),
                margin: UiRect::all(Val::Auto),
                overflow: Overflow::scroll_y(),
                ..Default::default()
            },
            BorderColor::all(Color::BLACK),
            BorderRadius::all(Val::Px(15.0)),
            BackgroundColor(Color::Srgba(bevy::color::palettes::css::TAN).lighter(0.30)),
            DespawnOnExit(Day(DAY as u8)),
            children![
                (
                    Node {
                        position_type: PositionType::Absolute,
                        height: Val::Percent(98.),
                        width: Val::Percent(2.),
                        margin: UiRect::vertical(Val::Auto),
                        left: Val::Percent(0.5),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(50.)),
                    BackgroundColor(Color::linear_rgb(1., 0., 0.)),
                ),
                (
                    Node {
                        position_type: PositionType::Absolute,
                        margin: UiRect::vertical(Val::Auto),
                        height: Val::Percent(98.0),
                        width: Val::Percent(2.),
                        right: Val::Percent(0.5),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(50.)),
                    BackgroundColor(Color::linear_rgb(0., 0., 0.)),
                ),
            ],
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    height: Val::Percent(100.),
                    width: Val::Percent(98.),
                    flex_grow: 1.0,
                    left: Val::Percent(2.5),
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ScrollPosition::default(),
                Interaction::default(),
            ))
            .with_children(|p| {
                banks.for_each(|b| {
                    p.spawn(b);
                });
            });
        });
    commands.trigger(Compute::<DAY>);
}

fn cleanup_day(mut commands: Commands) {}

fn solve_part1(
    _: On<Compute<DAY>>,
    input: Query<&Children, With<Bank>>,
    batterys: Query<&Battery>,
    mut answers: ResMut<Anwsers>,
) {
    let mut total_power = 0;
    for bank in &input {
        let mut b0 = 0;
        let mut b1 = batterys.get(*bank.last().unwrap()).unwrap().level();
        for battery in 0..bank.len() {
            let power = batterys
                .get(bank[battery])
                .expect("Battery in Bank")
                .level();
            if power > b0 && battery != bank.len() - 1 {
                b0 = power;
                b1 = 0;
            } else if power > b1 {
                b1 = power;
            }
        }
        total_power += b0 * 10 + b1;
    }
    answers.add(DAY, crate::state::Puzzle::Part1, total_power as u64);
}

fn solve_part2(
    _: On<Compute<DAY>>,
    input: Query<&Children, With<Bank>>,
    batterys: Query<&Battery>,
    mut answers: ResMut<Anwsers>,
) {
    let mut total_power = 0;
    let mut used_batterys = [0; 12];
    for bank in &input {
        used_batterys.fill(0);
        let mut left = bank.len();
        for battery in bank.iter() {
            let power = batterys.get(battery).expect("Battery in Bank").level();
            let mut zero = false;
            for used in used_batterys
                .iter_mut()
                // skip used batterys if there are less batterys left than slots
                .skip(12usize.saturating_sub(left))
            {
                // if we are using this battery then all slots further down need to be zeroed
                // as we can't use and earlier battery afeter a later one and increasing an earlier battery will always be better
                if zero {
                    *used = 0;
                } else if power > *used {
                    *used = power;
                    zero = true;
                }
            }
            left -= 1;
        }
        for i in 0..12 {
            total_power += used_batterys[11 - i] * 10_usize.pow(i as u32);
        }
    }
    answers.add(DAY, crate::state::Puzzle::Part2, total_power as u64);
}

fn spawn_visuals(mut commands: Commands) {}
// 17406 low
// 173904601285724 low

fn draw_number_line(
    banks: Query<&Children, With<Bank>>,
    batterys: Query<&Battery>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F1) {
        let bank = banks.iter().next().unwrap();
        let blank = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Px(10.)),
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(0., 0., 0.)),
                DespawnOnExit(Day(DAY as u8)),
            ))
            .id();
        let root = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.),
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Px(10.)),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(1., 0., 0.)),
                ChildOf(blank),
                NumberLine,
            ))
            .id();
        let w = Val::Percent(100.0 / bank.len() as f32);
        for battery in bank {
            let battery = batterys.get(*battery).unwrap();
            let val = battery.level();
            commands.spawn((
                Node {
                    width: w,
                    height: Val::Percent(40.),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(1., 1., 0.)),
                ChildOf(root),
                children![(Text::from(format!("{}", val)), TextColor(Color::BLACK))],
                Slot(0),
                BVal(battery.level() as u8),
            ));
        }
    }
}

#[derive(Component)]
struct NumberLine;

#[derive(Component, Default)]
struct Slot(u8);
#[derive(Component)]
struct BVal(u8);

fn init_12_slot(
    number_line: Single<&Children, With<NumberLine>>,
    mut numbers: Query<(&mut BackgroundColor, &mut Slot)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::F2) {
        return;
    }
    let mut i = 12;
    for cell in number_line.iter().rev().take(12) {
        let Ok(mut bg) = numbers.get_mut(cell) else {
            continue;
        };
        bg.0.0 = Color::linear_rgb(0., 1., 0.);
        bg.1.0 = i;
        i -= 1;
    }
}

fn scan(
    number_line: Single<&Children, With<NumberLine>>,
    mut numbers: Query<(&mut Slot, &mut BackgroundColor, &BVal)>,
    mut running: Local<bool>,
    input: Res<ButtonInput<KeyCode>>,
    mut current: Local<u8>,
    mut best: Local<u8>,
) {
    if input.just_pressed(KeyCode::F3) && !*running {
        *running = true;
        *current += 1;
        if *current > 12 {
            *current = 1;
        }
        *best = 13 - *current;
        println!("Starting scan for {}", *current);
    }
    if !*running {
        return;
    }
    for cell in number_line.iter().rev() {
        let Ok((slot, _, val)) = numbers.get_mut(cell) else {
            continue;
        };
        if slot.0 > *current {
            continue;
        }
        if slot.0 == 0 {
            continue;
        }
        let val = val.0;
        println!("Scanning slot {} with value {}", slot.0, val);
        for (i, cell1) in number_line.iter().rev().skip(*best as usize).enumerate() {
            let Ok((mut s, mut bg, v)) = numbers.get_mut(cell1) else {
                continue;
            };
            println!("  Comparing to current {} with value {}", val, v.0);
            if s.0 != 0 {
                println!("    Slot already used");
                *running = false;
                break;
            }
            if v.0 >= val {
                println!("    Found better!");
                s.0 = *current;
                bg.0 = Color::linear_rgb(0., 1., 0.);
                numbers.get_mut(cell).unwrap().0.0 = 0;
                *best += i as u8 + 1;
                break;
            }
            bg.0 = Color::linear_rgb(0., 1., 1.);
        }
        *running = false;
        for (i, cell) in number_line.iter().rev().enumerate() {
            let Ok((s, mut bg, _)) = numbers.get_mut(cell) else {
                continue;
            };
            if s.0 != 0 {
                continue;
            }
            if i < *best as usize {
                bg.0 = Color::linear_rgb(1., 1., 0.);
            } else {
                bg.0 = Color::linear_rgb(1., 0., 0.);
            }
        }
    }
}
