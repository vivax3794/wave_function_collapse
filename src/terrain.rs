use bevy::prelude::{AssetServer, Handle, Image};
use std::collections::HashMap;

use crate::algo::{ConnectionRules, TileCollection};

macro_rules! create_tile {
    ($weigth:expr; $name:expr => $uni_rule:expr) => {
        create_tile!($weigth; $name => $uni_rule; $uni_rule; $uni_rule; $uni_rule)
    };
    ($weigth:expr; $name:expr => $up_rule:expr; $down_rule:expr; $left_rule:expr; $right_rule:expr) => {
        (
            $name.to_string(),
            ConnectionRules {
                weigth: $weigth,
                up: $up_rule.to_string(),
                down: $down_rule.to_string(),
                left: $left_rule.to_string(),
                right: $right_rule.to_string(),
            },
        )
    };
}

pub fn load_tiles() -> TileCollection {
    TileCollection::new(HashMap::from([
        create_tile!(2000; "grass" => "misc"),
        create_tile!(500; "grass_1" => "misc"),
        create_tile!(500; "grass_2" => "misc"),
        create_tile!(2000; "flowers_1" => "misc"),
        create_tile!(2000; "flowers_2" => "misc"),
        // ROADS
        create_tile!(400; "road_hor" => "misc"; "misc"; "road_hor"; "road_hor"),
        create_tile!(400; "road_vert" => "road_vert"; "road_vert"; "misc"; "misc"),
        create_tile!(10; "road_left_end" => "misc"; "misc"; "misc"; "road_hor"),
        create_tile!(10; "road_right_end" => "misc"; "misc"; "road_hor"; "misc"),
        create_tile!(10; "road_top_end" => "misc"; "road_vert"; "misc"; "misc"),
        create_tile!(10; "road_bottom_end" => "road_vert"; "misc"; "misc"; "misc"),
        create_tile!(50; "road_BL_turn" => "misc"; "road_vert"; "road_hor"; "misc"),
        create_tile!(50; "road_BR_turn" => "misc"; "road_vert"; "misc"; "road_hor"),
        create_tile!(50; "road_TL_turn" => "road_vert"; "misc"; "road_hor"; "misc"),
        create_tile!(50; "road_TR_turn" => "road_vert"; "misc"; "misc"; "road_hor"),
        create_tile!(2; "road_BTL_T" => "road_vert"; "road_vert"; "road_hor"; "misc"),
        create_tile!(2; "road_BTR_T" => "road_vert"; "road_vert"; "misc"; "road_hor"),
        create_tile!(2; "road_LRB_T" => "misc"; "road_vert"; "road_hor"; "road_hor"),
        create_tile!(2; "road_LRT_T" => "road_vert"; "misc"; "road_hor"; "road_hor"),
        create_tile!(1; "road_plus" => "road_vert"; "road_vert"; "road_hor"; "road_hor"),
        // WALLS
        create_tile!(100; "wall_hor" => "misc"; "misc"; "wall_hor"; "wall_hor"),
        create_tile!(100; "wall_ver" => "wall_hor"; "wall_ver"; "misc"; "misc"),
        create_tile!(3; "wall_left_end" => "misc"; "misc"; "misc"; "wall_hor"),
        create_tile!(3; "wall_right_end" => "misc"; "misc"; "wall_hor"; "misc"),
        create_tile!(3; "wall_top_end" => "misc"; "wall_ver"; "misc"; "misc"),
        create_tile!(3; "wall_bottom_end" => "wall_ver"; "misc"; "misc"; "misc"),
        create_tile!(40; "wall_BR_turn" => "misc"; "wall_hor"; "misc"; "wall_hor"),
        create_tile!(40; "wall_BL_turn" => "misc"; "wall_hor"; "wall_hor"; "misc"),
        create_tile!(40; "wall_TR_turn" => "wall_hor"; "misc"; "misc"; "wall_hor"),
        create_tile!(40; "wall_TL_turn" => "wall_hor"; "misc"; "wall_hor"; "misc"),
        // WATER
        create_tile!(200; "water" => "water"),
        create_tile!(100; "water_left" => "water_left"; "water_left"; "misc"; "water"),
        create_tile!(100; "water_right" => "water_right"; "water_right"; "water"; "misc"),
        create_tile!(100; "water_top" => "misc"; "water"; "water_top"; "water_top"),
        create_tile!(100; "water_bottom" => "water"; "misc"; "water_bottom"; "water_bottom"),
        create_tile!(5; "water_TL" => "misc"; "water_left"; "misc"; "water_top"),
        create_tile!(5; "water_TR" => "misc"; "water_right"; "water_top"; "misc"),
        create_tile!(5; "water_BL" => "water_left"; "misc"; "misc"; "water_bottom"),
        create_tile!(5; "water_BR" => "water_right"; "misc"; "water_bottom"; "misc"),
        create_tile!(5; "water_edge_BL" => "water" ; "water_left"; "water_bottom"; "water"),
        create_tile!(5; "water_edge_BR" => "water" ; "water_right"; "water"; "water_bottom"),
        create_tile!(5; "water_edge_TL" => "water_left" ; "water"; "water_top"; "water"),
        create_tile!(5; "water_edge_TR" => "water_right" ; "water"; "water"; "water_top"),
    ]))
}

pub fn load_assets(assets: &AssetServer, names: Vec<String>) -> HashMap<String, Handle<Image>> {
    HashMap::from_iter(
        names
            .into_iter()
            .map(|name| (name.clone(), assets.load(&format!("images/{}.jpg", &name)))),
    )
}
