use bevy::{app::{Plugin, Update}, input::ButtonInput, prelude::{Component, KeyCode, Query, Res}};

#[derive(Component, Default)]
pub struct PlayerInput {
    pub thrust: bool,
    pub turn: f32,
    pub fire: bool,
}

fn update_player_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerInput>,
) {
    let thrust_input = keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let fire_input = keys.any_pressed([KeyCode::Space, KeyCode::ShiftRight]);

    let left_turn_input = if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) { -1.0 } else { 0.0 };
    let right_turn_input = if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) { 1.0 } else { 0.0 };
    let turn_input = left_turn_input + right_turn_input;

    for mut player_input in &mut query {
        player_input.thrust = thrust_input;
        player_input.fire = fire_input;
        player_input.turn = turn_input;
    }
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_player_input_system);
    }
}

