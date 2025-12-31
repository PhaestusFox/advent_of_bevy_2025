use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

#[derive(Resource)]
struct RawDay(String);

impl FromWorld for RawDay {
    fn from_world(_world: &mut World) -> Self {
        RawDay(include_str!("../../assets/days/day03.input").to_string())
    }
}

pub struct DayPlugin;
impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RawDay>();
        app.add_systems(Startup, setup_day);
        app.add_systems(
            Update,
            (solve_part1, solve_part2).run_if(input_just_pressed(KeyCode::Space)),
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

    fn level(&self) -> usize {
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

    fn color(&self) -> Color {
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

fn setup_day(input: Res<RawDay>, mut commands: Commands) {
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
}

fn solve_part1() {
    info!("Part 1: Not Solved");
}

fn solve_part2() {
    info!("Part 2: Not Solved");
}
// 17406 low
// 173904601285724 low
