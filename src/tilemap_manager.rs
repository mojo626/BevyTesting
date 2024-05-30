use std::borrow::{Borrow, BorrowMut};

use bevy::{asset::LoadState, prelude::*};
use bevy::reflect::TypePath;
use bevy_common_assets::json::{self, JsonAssetPlugin};

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct Tilemap {
    mapHeight: u32,
    mapWidth: u32,
    tileSize: u32,
    layers: Vec<Layer>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct Layer {
    collider: bool,
    name: String,
    tiles: Vec<Tile>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct Tile {
    id: String,
    x: u32,
    y: u32,
}

#[derive(Resource, Debug)]
struct TilemapHandle(Handle<Tilemap>);

pub async fn generate_tilemap_arr<'a>(asset_server: &Res<'a, AssetServer>, json_assets: & Res<'a, Assets<Tilemap>>) -> Vec<Vec2> {
    let data = TilemapHandle(asset_server.load("tilemaps/map.json").await);


    println!("{:?}", json_assets.get(&data.0).unwrap());

    let mut arr = Vec::new();

    arr.push(Vec2::new(0.0, 0.0));

    return arr;
}