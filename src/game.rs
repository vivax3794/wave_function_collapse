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

fn create_full_grid(mut commands: Commands, assets: Res<AssetServer>) {
    let tileset = terrain::load_tiles();
    let asset_names = tileset.0.keys().cloned().collect();

    println!("TILES: {:?} \n\n", tileset);
    let mut grid = Grid::new(tileset, 140, 80);
    grid.collapse_all();

    let assets = terrain::load_assets(&assets, asset_names);

    let grid: Vec<Vec<&Handle<Image>>> = grid
        .0
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|n| {
                    let name = n.get_tiles().iter().next().unwrap().clone();
                    assets
                        .get(&name)
                        .expect(&format!("could not find asset for {}", name))
                })
                .collect()
        })
        .collect();

    for x in 0..140 {
        for y in 0..80 {
            let img = grid[x][y];
            let pos = Transform::from_xyz((x as f32 - 70.0) * 8.0, (y as f32 - 40.0) * 8.0, 0.0);

            commands.spawn_bundle(SpriteBundle {
                texture: img.clone(),
                transform: pos,
                ..default()
            });
        }
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

fn step(mut commands: Commands, mut grid: ResMut<Grid>, assets: Res<Assets>) {
    for _ in 0..20 {
        if grid.1 == grid.0.len() * grid.0[0].len() {
            return;
        }

        let (x, y, tile_name) = grid.tick();

        let img = assets.0.get(&tile_name).unwrap();
        let pos = Transform::from_xyz((x as f32 - 80.0) * 8.0, (y as f32 - 40.0) * 8.0, 0.0);

        commands.spawn_bundle(SpriteBundle {
            texture: img.clone(),
            transform: pos,
            ..default()
        });
        // sleep(Duration::from_millis(20));
    }
}
