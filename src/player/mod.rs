use bevy::prelude::*;

pub enum BodyHitLocation {
    Head,
    RightArm,
    LeftArm,
    Body,
    RightLag,
    LeftLeg,
}

pub fn setup_player(commands: &mut Commands) {
    commands.spawn_bundle((
        Transform::default(),
        Velocity(Vec2::ZERO),
        Position(Vec3::ZERO),
    ));
}
