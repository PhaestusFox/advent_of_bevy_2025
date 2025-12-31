use bevy::{feathers::dark_theme::create_dark_theme, prelude::*};

mod days;

mod state;

mod book_keeping;

mod ui;

pub use book_keeping::Submit;
pub use days::*;
pub use state::Day;

pub struct AoCPlugin;

impl Plugin for AoCPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, bevy::feathers::FeathersPlugins));
        app.insert_resource(bevy_pkv::PkvStore::new("Phox", "AoB"));
        app.init_resource::<Seed>();
        app.add_plugins(days::DaysPlugin);
        app.init_state::<state::Day>();
        app.init_resource::<book_keeping::CurrentDayRaw>();
        app.insert_resource(Time::<Fixed>::from_hz(3.));
        app.add_plugins(ui::UIPlugin);
        app.add_plugins(calendar::CalendarPlugin);
        app.add_systems(Startup, spawn_camera);
        app.insert_resource(bevy::feathers::theme::UiTheme(create_dark_theme()));
        app.init_resource::<book_keeping::Anwsers>();
        app.add_observer(book_keeping::submit_answers);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(strum_macros::AsRefStr)]
pub enum PKVKeys {
    CalendarState25,
    Seed,
}

mod calendar;

pub use book_keeping::Seed;
