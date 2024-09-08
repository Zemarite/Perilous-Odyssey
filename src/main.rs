mod actors;
mod common;

use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Perilous Odessey".into(),
                        resolution: (940.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor_sys)
        .add_systems(Update, common::movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.50, 0.25, 0.25),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    },));
}

fn draw_cursor_sys(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
