

use std::array;

use bevy::{
    ecs::storage, math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle}, window::close_on_esc
};

use bevy_common_assets::json::JsonAssetPlugin;
use tilemap_manager::generate_tilemap_arr;


use crate::tilemap_manager::Tilemap;

mod tilemap_manager;

const PLAYER_SPEED: f32 = 2.0;
const PLAYER_SIZE: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Material2dPlugin::<TilemapMaterial>::default())
        .add_plugins(JsonAssetPlugin::<Tilemap>::new(&["map.json"]))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                update,
                check_for_collisions,
            ),
        )
        .run();
}


#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    

    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("sprites/player.png"),
            transform: Transform{
                scale: Vec3::new(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
    ));

    let mut arr:Vec<Vec2> = Vec::new();

    let arr2 = generate_tilemap_arr(8);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
            material: materials.add(
                TilemapMaterial {
                    color: Color::BLUE,
                    tilemap_texture: Some(asset_server.load("sprites/tilemap.png")),
                    tile_data: arr,
                    map_width: 10,
                }
            ),
            transform: Transform{
                scale: Vec3::new(200.0, 200.0, 1.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));
}


fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();

    if (keyboard_input.pressed(KeyCode::KeyA))
    {
        player_transform.translation.x -= PLAYER_SPEED;
    }
    if (keyboard_input.pressed(KeyCode::KeyD))
    {
        player_transform.translation.x += PLAYER_SPEED;
    }
}

fn check_for_collisions(
    mut player_query: Query<(&mut Transform, &Handle<Image>), With<Player>>,
    collider_query: Query<(Entity, &Transform), (With<Collider>, Without<Player>)>,
    assets: Res<Assets<Image>>,
) {
    let (player_transform, player_handle) = player_query.single_mut();

    for (collider_entity, transform) in &collider_query {
        let collision = Aabb2d::new(player_transform.translation.truncate(), assets.get(player_handle).unwrap().size().as_vec2() * player_transform.scale.truncate() / 2.0)
            .intersects(&Aabb2d::new(transform.translation.truncate(), transform.scale.truncate() / 2.0));

        //println!("{}", collision);
    }
}




#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct TilemapMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    tilemap_texture: Option<Handle<Image>>,
    #[storage(3)]
    tile_data: Vec<Vec2>,
    #[uniform(4)]
    map_width: u32,
}

impl Material2d for TilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap_shader.wgsl".into()
    }
}