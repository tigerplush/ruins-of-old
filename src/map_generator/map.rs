use std::cmp::{max, min};

use bevy::prelude::*;
use rand::Rng;
use bresenham::*;

use crate::common::{rect::Rect, TileType, WIDTH, HEIGHT, Vec2Int};

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
        let mut rng = rand::thread_rng();
        for _ in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let x = rng.gen_range(0..map.width - w - 2);
            let y = rng.gen_range(0..map.height - h - 2);
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.gen_range(0..=1) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y + 1..=room.y2 {
            for x in room.x + 1..=room.x2 {
                let index = self.xy_idx(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn generate_empty_tiles(&self) -> Vec<TileType> {
        let mut tiles = vec![TileType::Floor; 80 * 50];

        for x in 0..self.width {
            tiles[self.xy_idx(x, 0)] = TileType::Wall;
            tiles[self.xy_idx(x, self.height - 1)] = TileType::Wall;
        }
        for y in 0..self.height {
            tiles[self.xy_idx(0, y)] = TileType::Wall;
            tiles[self.xy_idx(self.width - 1, y)] = TileType::Wall;
        }
        tiles
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn is_visible(&self, start: Vec2Int, end: Vec2Int) -> bool {

        let line: Vec<_> = Bresenham::new((start.x as isize, start.y as isize), (end.x as isize, end.y as isize)).collect();

        for (x, y) in line {
            let idx = self.xy_idx(x as i32, y as i32);
            if self.tiles[idx] == TileType::Wall {
                return false;
            }
        }
        true
    }
}
