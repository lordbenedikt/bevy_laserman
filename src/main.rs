#![warn(unused_imports)]
#![warn(deprecated)]

use bevy::{
    prelude::*,
    render::{camera::*},
    reflect::TypeRegistry,
};
use bevy_rapier2d::prelude::*;

use consts::*;
use serde::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_egui::EguiPlugin;

mod anim;
mod consts;
mod misc;
mod level;
mod player;
mod save_load;
mod mesh_gen;
mod bevy_image;
mod my_egui;

#[derive(Resource)]
pub struct CursorPos(Vec2);

#[derive(Default)]
pub struct General {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        // REGISTER TYPES
        // CUSTOM ASSETS
        .add_plugin(RonAssetPlugin::<level::Level>::new(&["lvl"]))
        // RESOURCES
        .insert_resource(mesh_gen::ImageColliderMap::default())
        .insert_resource(CursorPos(Vec2::new(0.0, 0.0)))
        .insert_resource(player::Player::default())
        .insert_resource(level::PolygonInMaking::default())
        .insert_resource(level::AddPhysicsRequests::default())
        .insert_resource(my_egui::State::default())
        // SETUP
        .add_startup_system(misc::setup)
        .add_startup_system(level::setup)
        .add_startup_system(player::setup)
        // SYSTEMS
        .add_system_set(misc::system_set())
        .add_system_set(level::system_set().label("level"))
        .add_system_set(player::system_set().label("player"))
        .add_system_set(anim::system_set().label("anim").after("player"))
        .add_system_set(my_egui::system_set().label("egui"))
        .add_system(level::spawn_object_at_mouse)
        // .add_system_set(save_load::system_set().label("save_load"))
        .run()
}
