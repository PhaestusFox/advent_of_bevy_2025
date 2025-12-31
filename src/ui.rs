use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scroll_wheel);
    }
}

fn scroll_wheel(
    mut query: Query<(&mut ScrollPosition, &Interaction)>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
) {
    for (mut scroll_pos, interaction) in &mut query {
        if *interaction == Interaction::Hovered {
            scroll_pos.y += mouse_wheel.delta.y;
        }
    }
}
