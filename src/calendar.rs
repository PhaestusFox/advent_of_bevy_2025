use bevy::feathers::controls::{ButtonProps, ButtonVariant, button};
use bevy::feathers::cursor::EntityCursor;
use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::picking::hover::Hovered;
use bevy::ui::InteractionDisabled;
use bevy::{asset, prelude::*};
use chrono::Datelike;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::state::Puzzle;
use crate::{Day, PKVKeys, Seed};

#[derive(Resource, Serialize, Deserialize)]
pub(crate) struct CalendarState {
    days: Vec<DayState>,
}

impl FromWorld for CalendarState {
    fn from_world(world: &mut World) -> Self {
        let pkv = world.resource::<bevy_pkv::PkvStore>();
        if let Ok(save) = pkv.get(PKVKeys::CalendarState25) {
            save
        } else {
            CalendarState {
                days: vec![
                    DayState {
                        puzzle1_completed: false,
                        puzzle2_completed: false,
                    };
                    25
                ],
            }
        }
    }
}

impl CalendarState {
    pub fn pass(&mut self, day: u8, puzzle: Puzzle) {
        let day_state = &mut self.days[(day - 1) as usize];
        match puzzle {
            Puzzle::Part1 => day_state.puzzle1_completed = true,
            Puzzle::Part2 => day_state.puzzle2_completed = true,
        }
    }
}

pub struct CalendarPlugin;

impl Plugin for CalendarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CalendarState>();
        app.add_systems(OnEnter(Day(0)), spawn_calendar);
        app.add_systems(OnExit(Day(0)), spawn_back_to_calendar);
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct DayState {
    puzzle1_completed: bool,
    puzzle2_completed: bool,
}

fn spawn_calendar(
    mut commands: Commands,
    calendar_state: Res<CalendarState>,
    assets: Res<AssetServer>,
    seed: Res<Seed>,
) {
    let today = chrono::Utc::now().day() as u8;
    let mut days = (1..=25).collect::<Vec<_>>();
    days.shuffle(&mut rand::rngs::StdRng::seed_from_u64(seed.0));
    commands
        .spawn((
            Node {
                height: Val::Percent(80.0),
                margin: UiRect::all(Val::Auto),
                display: Display::Grid,
                aspect_ratio: Some(1.),
                flex_direction: FlexDirection::Row,
                grid_auto_flow: GridAutoFlow::Column,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceAround,
                align_content: AlignContent::SpaceEvenly,
                row_gap: Val::Percent(4.),
                column_gap: Val::Percent(4.),
                padding: UiRect::all(Val::Percent(2.)),
                grid_template_rows: vec![RepeatedGridTrack::percent(5, 16.)],
                grid_template_columns: vec![RepeatedGridTrack::percent(5, 16.)],
                ..Default::default()
            },
            BorderRadius::all(Val::Px(15.)),
            DespawnOnExit(Day(0)),
            BackgroundGradient(vec![Gradient::Linear(LinearGradient::to_bottom_right(
                vec![
                    ColorStop::new(Color::hsl(0., 0.7, 0.4), Val::Percent(0.0)),
                    ColorStop::new(Color::hsl(60., 0.7, 0.4), Val::Percent(50.0)),
                    ColorStop::new(Color::hsl(120., 0.7, 0.4), Val::Percent(100.0)),
                ],
            ))]),
            TabGroup::new(0),
        ))
        .with_children(|p| {
            for day in days {
                let color = if calendar_state.days[(day - 1) as usize].puzzle2_completed {
                    Color::linear_rgb(0.827, 0.69, 0.216)
                } else if calendar_state.days[(day - 1) as usize].puzzle1_completed {
                    Color::linear_rgb(0.71, 0.718, 0.733)
                } else {
                    Color::linear_rgb(0.2, 0.2, 0.2)
                };
                let mut button = p.spawn(button(
                    ButtonProps {
                        variant: bevy::feathers::controls::ButtonVariant::Primary,
                        corners: bevy::feathers::rounded_corners::RoundedCorners::All,
                    },
                    (
                        Day(day),
                        ImageNode {
                            image: assets.load("star.png"),
                            color,
                            ..Default::default()
                        },
                        BackgroundColor(Color::hsl(0., 0.7, 0.4)),
                    ),
                    Spawn((Text::from(format!("{}", day)),)),
                ));
                button.observe(change_day);
                button.insert((
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Px(15.)),
                    TabIndex(day as i32),
                ));
                if day > today || day > 7 {
                    button.insert(InteractionDisabled);
                }
            }
        });
}

fn change_day(event: On<bevy::ui_widgets::Activate>, day: Query<&Day>, mut commands: Commands) {
    let Ok(day) = day.get(event.entity) else {
        warn!("Change day observer add to entity without Day component");
        return;
    };
    println!("Changing to day {:?}", day);
    commands.trigger(*day);
}

fn spawn_back_to_calendar(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            ImageNode {
                image: assets.load("X.png"),
                color: Color::hsl(0., 1., 0.4),
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Percent(2.5),
                aspect_ratio: Some(1.0),
                min_height: Val::Px(32.0),
                min_width: Val::Px(32.0),
                border: UiRect::all(Val::Px(3.)),
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(0.7, 0.7, 0.7)),
            BorderRadius::all(Val::Px(10.)),
            BorderColor::all(Color::BLACK),
            Hovered::default(),
            EntityCursor::System(bevy::window::SystemCursorIcon::ContextMenu),
            (
                DespawnOnEnter(Day(0)),
                Day(0),
                Button,
                TabIndex(0),
                GlobalZIndex(1),
            ),
        ))
        .observe(
            |click: On<Pointer<Click>>, mut next: ResMut<NextState<Day>>| {
                if click.button == PointerButton::Primary {
                    next.set(Day(0));
                }
            },
        );
}
