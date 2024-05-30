use std::borrow::{Borrow, BorrowMut};
use std::fs;

use bevy::asset::LoadContext;
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

pub fn generate_tilemap_arr(tileset_width: u32) -> Vec<Vec2> {
    //read the json of the tilemap
    //using https://www.spritefusion.com/editor as tilemap editor
    let json_contents = fs::read_to_string("assets/tilemaps/map.json").expect("Couldn't read JSON");
    let data : Tilemap = serde_json::from_str(&json_contents).unwrap();


    for layer in data.layers.into_iter().rev() {
        for tile in layer.tiles {
            //the position that we are at in the image
            let xPos = tile.x;
            let yPos = tile.y;

            //the position that we are at in the tileset
            let tileX = tile.id.parse::<u32>().unwrap() % tileset_width;
            let tileY = (tile.id.parse::<u32>().unwrap() - (tile.id.parse::<u32>().unwrap() % tileset_width)) / tileset_width;

            println!("{}, {}", tileX, tileY);
        }
    }

    let mut arr = Vec::new();

    arr.push(Vec2::new(0.0, 0.0));

    return arr;
}