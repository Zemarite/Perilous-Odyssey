use bevy::prelude::*;
use bevy_inspector_egui::prelude::InspectorOptions;

#[derive(Component, InspectorOptions, Debug, Default)]
pub struct Velocity(Vec2);
#[derive(Component, InspectorOptions, Debug, Default)]
pub struct Position(Vec3);

pub fn movement_system(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0.extend(0.0) * time.delta_seconds(); // Update position based on velocity
    }
}
