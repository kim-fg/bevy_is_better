use bevy::prelude::Component;

#[derive(Component)]
pub struct Ship {
    pub thrust_power: f32,
    pub brake_power: f32,
    pub turn_speed: f32,
}