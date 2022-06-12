use crate::algo::Grid;
use crate::terrain;
use bevy::prelude::*;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

/// main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(create_full_grid);
        app.add_startup_system(setup_step_by_step).add_system(step);
    }
}

struct Assets(HashMap<String, Handle<Image>>);

fn setup_step_by_step(mut commands: Commands, assets: Res<AssetServer>) {
    let tileset = terrain::load_tiles();
    let asset_names = tileset.0.keys().cloned().collect();
    println!("TILES: {:?} \n\n", tileset);
    let grid = Grid::new(tileset, 160, 80);

    let assets = terrain::load_assets(&assets, asset_names);

    commands.insert_resource(grid);
    commands.insert_resource(Assets(assets));
}

fn step(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    assets: Res<Assets>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        let position = position / 8.0;
        println!("{}", position);
        for _ in 0..4 {
            if let Some((x, y, tile_name)) = grid.tick_in_area(
                (position.x.max(10.0) as usize - 10)..(position.x.min(149.0) as usize + 10),
                (position.y.max(10.0) as usize - 10)..(position.y.min(69.0) as usize + 10),
            ) {
                let img = assets.0.get(&tile_name).unwrap();
                let pos =
                    Transform::from_xyz((x as f32 - 80.0) * 8.0, (y as f32 - 40.0) * 8.0, 0.0);

                commands.spawn_bundle(SpriteBundle {
                    texture: img.clone(),
                    transform: pos,
                    ..default()
                });
                // sleep(Duration::from_millis(20));
            }
        }
    }
}
