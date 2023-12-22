use std::{collections::{HashMap, BinaryHeap, VecDeque}, cmp::Ordering};
use bevy::prelude::*;

use crate::{common::Vec2Int, map_generator::Map};

#[derive(Component, Debug, PartialEq)]
pub struct Path {
    pub waypoints: VecDeque<Vec2Int>,
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    position: Vec2Int,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {
    pub fn calculate(start: Vec2Int, target: Vec2Int, map: &Map) -> Option<Path> {
        let mut heads = BinaryHeap::new();
        heads.push(Node {
            cost: 0,
            position: start,
        });
    
        let mut came_from: HashMap<Vec2Int, Option<Vec2Int>> = HashMap::new();
        came_from.insert(start, None);
        let mut cost_so_far = HashMap::new();
        cost_so_far.insert(start, 0);
    
        while let Some(head) = heads.pop() {
            if head.position == target {
                break;
            }
            for direction in Vec2Int::DIRECTIONS {
                let next = head.position + direction;
                let idx = map.xy_idx(next.x, next.y);
                if map.is_occupied(idx) {
                    continue;
                }
                let new_cost = cost_so_far.get(&head.position).unwrap() + cost(head.position, next);
                if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                    cost_so_far.insert(next, new_cost);
                    let priority = new_cost + next.distance(&target).round() as i32;
                    heads.push(Node {
                        cost: priority,
                        position: next,
                    });
                    came_from.insert(next, Some(head.position));
                }
            }
        }
    
        let mut waypoints = Vec::new();
        let mut current = target;
        while let Some(previous) = came_from.get(&current) {
            if let Some(p) = previous {
                waypoints.push(current);
                current = *p;
            }
            else {
                break;
            }
        }
    
        if waypoints.is_empty() {
            return None;
        }
        Some(Path {
            waypoints: waypoints.iter().copied().rev().collect()
        })
    }
}


fn cost(_: Vec2Int, _: Vec2Int) -> i32 {
    0
}

#[test]
fn test_pathfinding() {
    let map = Map::new();
    let start = map.rooms.iter().nth(0).unwrap().center();
    let target = map.rooms.iter().nth(1).unwrap().center();
    let path = Path::calculate(Vec2Int::new(start.0, start.1), Vec2Int::new(target.0, target.1), &map);
    println!("start: {:?}, end: {:?}", start, target);
    println!("{:?}", path);
    assert!(path.is_some());
    let dead_end = Vec2Int::new(0, 0);
    let path = Path::calculate(Vec2Int::new(start.0, start.1), dead_end, &map);
    assert_eq!(path, None);
}