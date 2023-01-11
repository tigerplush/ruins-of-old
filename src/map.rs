use rltk::{Rltk, RGB, RandomNumberGenerator, Point, Algorithm2D, BaseMap};
use super::{Rect};
use std::cmp::{min, max};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn new() -> Self {
        let mut map = Self {
            tiles: vec![TileType::Wall; 80*50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80*50],
            visible_tiles: vec![false; 80*50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();
        
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
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
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    }
                    else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);

                    }
                }
                map.rooms.push(new_room);
            }
        }

        map
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.xy_idx(x, y);

                if self.revealed_tiles[idx] {
                    let glyph;
                    let mut fg;
                    match self.tiles[idx] {
                        TileType::Floor => {
                            glyph = rltk::to_cp437('.');
                            fg = RGB::from_f32(0.0, 0.5, 0.5);
                        },
                        TileType::Wall => {
                            glyph = rltk::to_cp437('#');
                            fg = RGB::from_f32(0.0, 1.0, 0.0);
                        },
                    }
                    if !self.visible_tiles[idx] {
                        fg = fg.to_greyscale();
                    }
                    ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
                }
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}