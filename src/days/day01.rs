use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    Day, Submit,
    book_keeping::{Anwsers, CurrentDayRaw},
    days::Compute,
};

const DAY: usize = 1;

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        // Setup for Day 1 goes here
        app.add_systems(OnEnter(Day(DAY as u8)), (setup_day, spawn_visuals));
        app.add_observer(solve_part1);
        app.add_observer(solve_part2);
        app.add_systems(OnExit(Day(DAY as u8)), cleanup_day);
        app.add_systems(Update, (spin_dial.after(send_message_per_frame),));
        app.add_systems(
            FixedUpdate,
            (
                send_message_per_frame.run_if(resource_exists::<Steps>),
                update_dial,
            )
                .chain(),
        );
        app.add_observer(send_steps);

        app.add_message::<Step>();
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Steps {
    #[deref]
    list: Vec<i32>,
    taken: usize,
    per_frame: bool,
}

#[derive(Debug, Clone, Copy, Message, Deref)]
pub struct Step(i32);

#[derive(Component, Deref, DerefMut)]
struct Dial {
    #[deref]
    end: i32,
    per_frame: f32,
}

#[derive(Event)]
pub enum Request {
    TogglePerFrame,
    Steps(usize),
    Reset,
}

fn send_steps(
    request: On<Request>,
    mut steps: ResMut<Steps>,
    mut message_writer: MessageWriter<Step>,
) {
    match request.event() {
        Request::TogglePerFrame => {
            steps.per_frame = !steps.per_frame;
        }
        Request::Steps(send) => {
            for step in steps.list.iter().skip(steps.taken).take(*send) {
                message_writer.write(Step(*step));
            }
            steps.taken += *send;
        }
        Request::Reset => {
            steps.taken = 0;
        }
    }
}

#[derive(Component)]
struct Arrow;

fn parse_input(input: &str) -> Steps {
    let mut steps = Vec::new();
    for line in input.lines() {
        if let Ok(step) = line.trim()[1..].parse::<i32>() {
            if line.starts_with('R') {
                steps.push(step);
            } else if line.starts_with('L') {
                steps.push(-step);
            } else {
                error!("Invalid direction in line: {}", line);
            }
        } else {
            error!("Failed to parse line: {}", line);
        }
    }
    Steps {
        list: steps,
        taken: 0,
        per_frame: true,
    }
}

fn setup_day(input: Res<CurrentDayRaw>, mut commands: Commands) {
    commands.insert_resource(parse_input(&input.0));
}

fn cleanup_day(mut commands: Commands) {
    commands.remove_resource::<Steps>();
}

fn solve_part1(
    _: On<Compute<DAY>>,
    steps: Res<Steps>,
    mut answers: ResMut<Anwsers>,
    mut commands: Commands,
) {
    let mut zeros = 0;
    let mut index = 50;
    for step in steps.iter() {
        index += step;
        index %= 100;
        if index == 0 {
            zeros += 1;
        }
    }
    answers.add(DAY, crate::state::Puzzle::Part1, zeros);
    commands.trigger(Submit::Part1(zeros));
}

fn solve_part2(
    _: On<Compute<DAY>>,
    steps: Res<Steps>,
    mut answers: ResMut<Anwsers>,
    mut commands: Commands,
) {
    let mut index: i32 = 50;
    let mut index2 = 50;
    let mut passed = 0;
    let mut passed2 = 0;
    for (i, step) in steps.iter().enumerate() {
        // count the number of times we pass zero this lap
        let full = step.abs() / 100;
        // add the number of full laps to zeros passed
        passed2 += full;

        // store if we are left or right of zero before the step
        let is_pos = index.is_positive();
        index2 += step % 100;
        if index2.is_positive() != is_pos {
            passed2 += 1;
        }
        if index2 == 0 {
            passed2 += 1;
        } else if index2 > 99 {
            passed2 += 1;
            index2 -= 100;
        } else if index2 < 0 {
            passed2 += 1;
            index2 += 100;
        }
        for _ in 0..step.abs() {
            index += step.signum();
            index %= 100;
            if index == 0 {
                passed += 1;
            }
        }
    }
    info!("Part 2: Final position: {}", passed);
    answers.add(DAY, crate::state::Puzzle::Part2, passed);
    commands.trigger(Submit::Part2(passed));
}

fn send_message_per_frame(mut message_writer: MessageWriter<Step>, mut steps: ResMut<Steps>) {
    if steps.taken < steps.list.len() && steps.per_frame {
        let step = steps.list[steps.taken];
        message_writer.write(Step(step));
        steps.taken += 1;
    }
}

fn spawn_visuals(
    window: Single<&Window>,
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let size = window.width().min(window.height());
    let circle = meshs.add(Circle::new(size * 0.33));
    let arrow = meshs.add(Triangle2d::default());
    let silver = colors.add(Color::linear_rgb(0.71, 0.718, 0.733));
    let red = colors.add(Color::linear_rgb(0.827, 0.216, 0.216));
    commands.spawn((
        Dial {
            end: 50,
            per_frame: 0.0,
        },
        Sprite {
            image: asset_server.load("dial.png"),
            custom_size: Some(Vec2::splat(0.6 * size)),
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_rotation_z(PI)),
    ));
    commands.spawn((
        Mesh2d(arrow),
        MeshMaterial2d(red),
        Arrow,
        Transform::from_translation(Vec3::new(0., 0.3 * size, 1.0))
            .with_scale(Vec3::new(33.3, 75., 100.))
            .with_rotation(Quat::from_rotation_z(PI)),
    ));
}

fn update_dial(
    mut dial: Single<(&mut Transform, &mut Dial)>,
    mut step_reader: MessageReader<Step>,
    fixed: Res<Time<Fixed>>,
) {
    for step in step_reader.read() {
        let radians = 0.02 * PI * (dial.1.end as f32);
        dial.0.rotation = Quat::from_rotation_z(radians);
        dial.1.end += **step;
        let radians = 0.02 * PI * (**step as f32);
        dial.1.per_frame = radians / (fixed.timestep().as_secs_f32() * 60.0);
    }
}

/// viewer is supposed to make this system
fn spin_dial(
    dial: Single<(&mut Transform, &Dial)>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    arrow: Single<&MeshMaterial2d<ColorMaterial>, With<Arrow>>,
) {
    let (mut transform, dial) = dial.into_inner();
    transform.rotation *= Quat::from_rotation_z(dial.per_frame);
    if let Some(material) = colors.get_mut(&arrow.0) {
        let current_angle = transform.rotation.to_euler(EulerRot::ZXY).0;
        if current_angle.abs() < PI / 100. {
            material.color = Color::linear_rgb(0.21, 0.718, 0.721);
        } else {
            material.color = Color::linear_rgb(0.827, 0.216, 0.216);
        }
    }
}
