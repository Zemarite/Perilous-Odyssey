use bevy::prelude::*;

fn mouse_input_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_events: EventReader<CursorMoved>,
    mut query: Query<&mut Velocity>,
) {
    for mut velocity in query.iter_mut() {
        // Reset velocity to zero unless mouse button 1 is pressed
        if !mouse_button_input.pressed(MouseButton::Left) {
            velocity.0 = Vec2::ZERO;
            continue;
        }

        // Get latest cursor position
        if let Some(cursor_event) = cursor_events.iter().last() {
            // Adjust velocity based on mouse movement
            let cursor_position = cursor_event.position;
            let center = Vec2::new(400.0, 300.0); // Example center of the screen
            let direction = (cursor_position - center).normalize(); // Adjust as needed
            velocity.0 = direction;
        }
    }
}
