use std::{
    collections::{HashMap, HashSet, VecDeque},
    slice::Iter,
};

use itertools::Itertools;
use rand::prelude::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ConnectionRules {
    pub weigth: usize,
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}

impl ConnectionRules {
    fn get(&self, dir: &Direction) -> &String {
        match dir {
            Direction::Up => &self.up,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }

    fn get_mut(&mut self, dir: &Direction) -> &mut String {
        match dir {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TileCollection(pub HashMap<String, ConnectionRules>);

impl TileCollection {
    pub fn new(map: HashMap<String, ConnectionRules>) -> Self {
        TileCollection(map)
    }

    fn entorpy(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Tile {
    Collapsed(String, ConnectionRules),
    Wave(TileCollection),
}

impl Tile {
    fn new(tiles: TileCollection) -> Self {
        Tile::Wave(tiles)
    }

    fn collapse(&mut self) {
        match self {
            Tile::Collapsed(_, _) => panic!("Cant collapse a already collapsed tile!"),
            Tile::Wave(options) => {
                let (name, rules) = options
                    .0
                    .iter()
                    .flat_map(|(name, rules)| vec![(name, rules); rules.weigth])
                    .choose(&mut thread_rng())
                    .expect("Empty wave!");
                *self = Tile::Collapsed(name.clone(), rules.clone());
            }
        }
    }

    fn is_wave(&self) -> bool {
        match self {
            Tile::Collapsed(_, _) => false,
            Tile::Wave(_) => true,
        }
    }

    pub fn get_tiles(&self) -> HashSet<String> {
        match self {
            Tile::Collapsed(name, _) => HashSet::from([name.clone()]),
            Tile::Wave(tiles) => tiles.0.keys().cloned().collect(),
        }
    }

    fn get_sockets_for(&self, dir: &Direction) -> Vec<String> {
        match self {
            Tile::Collapsed(_name, rules) => vec![rules.get(dir).clone()],
            Tile::Wave(tiles) => tiles
                .0
                .values()
                .map(|rules| rules.get(dir).clone())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Grid(pub Vec<Vec<Tile>>, pub usize);

impl Grid {
    pub fn new(tiles: TileCollection, width: usize, height: usize) -> Self {
        Grid(vec![vec![Tile::new(tiles); height]; width], 0)
    }

    fn find_lowest_entropy(&self) -> (usize, usize) {
        let cords = Itertools::cartesian_product(0..self.0.len(), 0..self.0[0].len())
            .filter(|(x, y)| matches!(self.0[*x][*y], Tile::Wave(_)));

        let min_enp = cords
            .clone()
            .map(|(x, y)| match &self.0[x][y] {
                Tile::Collapsed(_, _) => panic!("This should not be possible."),
                Tile::Wave(map) => map.entorpy(),
            })
            .min()
            .unwrap();
        cords
            .filter(|(x, y)| match &self.0[*x][*y] {
                Tile::Collapsed(_, _) => false,
                Tile::Wave(map) => map.entorpy() == min_enp,
            })
            .choose(&mut thread_rng())
            .unwrap()
    }

    // this gets NON COLLAPSED neighbours
    fn get_neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize, Direction)> {
        let mut possible = vec![(x + 1, y, Direction::Left), (x, y + 1, Direction::Down)];

        if x != 0 {
            possible.push((x - 1, y, Direction::Right))
        }
        if y != 0 {
            possible.push((x, y - 1, Direction::Up))
        }

        possible
            .into_iter()
            .filter(|(nx, ny, _)| {
                nx < &self.0.len() && ny < &self.0[0].len() && self.0[*nx][*ny].is_wave()
            })
            .collect()
    }

    fn propagate(&mut self, changed_tile: (usize, usize)) {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(changed_tile);

        while let Some((x, y)) = queue.pop_front() {
            for (nx, ny, dir) in self.get_neighbours((x, y)) {
                let mut changed = false;
                let sockets = self.0[x][y].get_sockets_for(&dir.opposite());
                if let Tile::Wave(n_tiles) = &mut self.0[nx][ny] {
                    n_tiles.0.retain(|_name, n_rules| {
                        let result = sockets.contains(n_rules.get(&dir));
                        if !result {
                            changed = true;
                        }
                        result
                    })
                }
                if changed {
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    pub fn tick(&mut self) -> (usize, usize, String) {
        let (x, y) = self.find_lowest_entropy();
        self.0[x][y].collapse();
        self.propagate((x, y));
        self.1 += 1;

        (x, y, self.0[x][y].get_tiles().into_iter().next().unwrap())
    }

    pub fn collapse_all(&mut self) {
        while self.1 != self.0.len() * self.0[0].len() {
            self.tick();
        }
    }
}
