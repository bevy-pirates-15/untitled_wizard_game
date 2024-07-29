//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
// use bevy_ecs_tilemap::prelude::*;

use super::{borders::SpawnBorders, player::SpawnPlayer, wand::SpawnWand};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct WorldBox;

// #[derive(Default, Debug, Resource)]
// struct ChunkManager {
//     pub spawned_chunks: HashSet<IVec2>,
// }


fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
) {
    // Spawn level box here, change to very pretty art later
    // commands.spawn((
    //     Name::new("World Box"),
    //     WorldBox,
    //     MaterialMesh2dBundle {
    //         // IMPORTANT: The mesh itself needs to be {1.0, 1.0} (default)
    //         // Only use "Transform" to manipulate tranform
    //         // Otherwise, the math is all off :(
    //         mesh: Mesh2dHandle(meshes.add(Rectangle::default())),
    //         transform: Transform::default()
    //             .with_scale(Vec2::new(MAP_WIDTH, MAP_HEIGHT).extend(0.0)),
    //         material: materials.add(Color::from(GREEN)),
    //         ..default()
    //     },
    //     StateScoped(Screen::Playing),
    // ));
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnBorders);
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnWand);
    // commands.trigger(StartWave);
}

// const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 256.0, y: 256.0 };
// // For this example, don't choose too large a chunk size.
// const CHUNK_SIZE: UVec2 = UVec2 { x: 2, y: 2 };
// // Render chunk sizes are set to 4 render chunks per user specified chunk.
// const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
//     x: CHUNK_SIZE.x * 2,
//     y: CHUNK_SIZE.y * 2,
// };

// const CENTER_TILE_POSITION: TilePos = TilePos { x: 0, y: 0};

// #[derive(Component)]
// struct TileEntity;

// fn spawn_chunk(commands: &mut Commands, images: Res<ImageAssets>,) {
//     let tilemap_entity = commands.spawn_empty().id();
//     let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

//     let transform = Transform::from_translation(Vec3::new(
//         CHUNK_SIZE.x as f32 * TILE_SIZE.x,
//         CHUNK_SIZE.y as f32 * TILE_SIZE.y,
//         0.0,
//     ));
    
//     // Spawn the elements of the tilemap.
//     for x in 0..CHUNK_SIZE.x {
//         for y in 0..CHUNK_SIZE.y {
//             let tile_pos = TilePos { x, y };

//             let tile_index = if tile_pos == CENTER_TILE_POSITION {
//                 3
//             } else if tile_pos.x == CENTER_TILE_POSITION.x {
//                 1
//             } else if tile_pos.y == CENTER_TILE_POSITION.y {
//                 2
//             } else {
//                 0
//             };

//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: tile_pos,
//                     tilemap_id: TilemapId(tilemap_entity),
//                     texture_index: TileTextureIndex(tile_index),
//                     ..Default::default()
//                 })
//                 .id();
//             commands.entity(tilemap_entity).add_child(tile_entity);
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }
//     let tilemap_texture = TilemapTexture::Single(images[&ImageAsset::MapTileset].clone_weak());

//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size: TILE_SIZE.into(),
//         size: CHUNK_SIZE.into(),
//         storage: tile_storage,
//         tile_size: TILE_SIZE,
//         texture: TilemapTexture::Single(images[&ImageAsset::Wizard].clone_weak()),
//         transform,
//         render_settings: TilemapRenderSettings {
//             render_chunk_size: RENDER_CHUNK_SIZE,
//             ..Default::default()
//         },
//         ..Default::default()
//     }).insert(TileEntity);
// }

// fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
//     let camera_pos = camera_pos.as_ivec2();
//     let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
//     let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
//     camera_pos / (chunk_size * tile_size)
// }

// fn spawn_chunks_around_camera(
//     mut commands: Commands,
//     images: Res<ImageAssets>,
//     camera_query: Query<&Transform, With<Camera>>,
//     mut chunk_manager: ResMut<ChunkManager>,
// ) {
//     if let Ok(transform) = camera_query.get_single() {
//         let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
//         for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
//             for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
//                 if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
//                     chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
//                     spawn_chunk(&mut commands, &images, IVec2::new(x, y));
//                 }
//             }
//         }
//     }
// }

// fn despawn_outofrange_chunks(
//     mut commands: Commands,
//     camera_query: Query<&Transform, With<Camera>>,
//     chunks_query: Query<(Entity, &Transform), With<TileEntity>>,
//     mut chunk_manager: ResMut<ChunkManager>,
// ) {
//     if let Ok(camera_transform) = camera_query.get_single() {
//         for (entity, chunk_transform) in chunks_query.iter() {
//             let chunk_pos = chunk_transform.translation.xy();
//             let distance = camera_transform.translation.xy().distance(chunk_pos);
//             if distance > 1024.0 {
//                 let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
//                 let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
//                 chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
//                 commands.entity(entity).despawn_recursive();
//             }
//         }
//     }
// }