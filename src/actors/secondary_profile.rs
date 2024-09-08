use bevy::prelude::*;
use bevy_inspector_egui::prelude::InspectorOptions;

#[derive(Bundle)]
pub struct SecondaryProfile {
    pub attacks: Attacks,
    pub wounds: Wounds,
    pub strength_bonus: StrengthBonus,
    pub toughness_bonus: ToughnessBonus,
    pub movement: Movement,
    pub magic: Magic,
}

#[derive(Component, InspectorOptions, Default)]
pub struct Attacks(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Wounds(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct StrengthBonus(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct ToughnessBonus(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Movement(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Magic(pub i32);

// #[derive(Component , InspectorOptions, Default)]
// pub struct InsanityPoints(pub i32);

// #[derive(Component , InspectorOptions, Default)]
// pub struct Fellowship(pub i32);
