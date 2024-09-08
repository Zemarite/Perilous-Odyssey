use bevy::prelude::*;
use bevy_inspector_egui::prelude::InspectorOptions;

// region: Main Profile

#[derive(Bundle)]
pub struct MainProfile {
    pub weapon_skill: WeaponSkill,
    pub ballistic_skill: BallisticSkill,
    pub strength: Strength,
    pub toughness: Toughness,
    pub agility: Agility,
    pub intelligence: Intelligence,
    pub will_power: WillPower,
    pub fellowship: Fellowship,
}

#[derive(Component, InspectorOptions, Debug, Default)]
pub struct WeaponSkill(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct BallisticSkill(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Strength(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Toughness(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Agility(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Intelligence(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct WillPower(pub i32);

#[derive(Component, InspectorOptions, Default)]
pub struct Fellowship(pub i32);
