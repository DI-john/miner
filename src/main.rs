//  Copyright 2022 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#![warn(clippy::all, clippy::pedantic)]

use bevy::{core::FixedTimestep, math::ivec3, prelude::*};

use bevy_simple_tilemap::prelude::*;

mod constants;
use constants::*;

mod model;
use model::elevator::Elevator;
use model::map::{Map, TileType};
use model::player::Player;

mod systems;

fn main() {
    App::new()
        // Disable MSAA, as it produces weird rendering artifacts
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(Elevator::new(MAX_ELEVATOR_DEPTH))
        .insert_resource(Player::new(PLAYER_START_X, PLAYER_START_Y))
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleTileMapPlugin)
        .add_startup_system(setup)
        .add_system(systems::input::camera_input)
        .add_system(systems::input::elevator_input)
        .add_system(systems::input::player_input)
        .add_system(systems::player::move_player)
        .add_system(systems::elevator::move_elevator.with_run_criteria(FixedTimestep::step(0.1)))
        .add_system(systems::render::update_tilemap)
        .add_system(systems::render::show_player)
        .add_system(systems::render::show_elevator)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load tilesheet texture and make a texture atlas from it
    let texture_handle = asset_server.load("64x64_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut cam = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(cam);

    let tilemap = TileMap::default();
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            scale: Vec3::splat(1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn_bundle(tilemap_bundle);

    let map = Map::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    commands.insert_resource(map);
}
