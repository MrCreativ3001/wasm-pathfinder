use crate::pathfinders::breadth_first::BreadthFirst;
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Add;

pub type Unit = i32;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Pos {
    pub x: Unit,
    pub y: Unit,
}
impl Pos {
    const UP: Pos = Pos { x: 0, y: -1 };
    const DOWN: Pos = Pos { x: 0, y: 1 };
    const LEFT: Pos = Pos { x: -1, y: 0 };
    const RIGHT: Pos = Pos { x: 1, y: 0 };
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    None,
    Wall,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2d<T> {
    width: usize,
    height: usize,
    flattened: Vec<T>,
}

impl<T> Vec2d<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        let flattened = vec![value; height * width];
        Self {
            width,
            height,
            flattened,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        let i = self.index(pos)?;
        Some(&self.flattened[i])
    }
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        let i = self.index(pos).expect("invalid position");
        Some(&mut self.flattened[i])
    }
    pub fn set(&mut self, pos: Pos, value: T) {
        let i = self.index(pos);
        let i = match i {
            Some(i) => i,
            None => return,
        };
        self.flattened[i] = value;
    }

    fn index(&self, pos: Pos) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width + x)
    }
}

impl<T> Vec2d<T>
where
    T: PartialEq,
{
    pub fn contains(&self, value: T) -> bool {
        self.flattened.contains(&value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    tiles: Vec2d<Tile>,
    start: Pos,
    end: Pos,
}

impl Grid {
    pub fn new(width: Unit, height: Unit, start: Pos, end: Pos) -> Self {
        Self {
            tiles: Vec2d::new(width as usize, height as usize, Tile::None),
            start,
            end,
        }
    }

    pub fn width(&self) -> Unit {
        self.tiles.width() as Unit
    }
    pub fn height(&self) -> Unit {
        self.tiles.height() as Unit
    }

    pub fn tile(&self, pos: Pos) -> Tile {
        *self.tiles.get(pos).expect("invalid position")
    }
    pub fn tile_opt(&self, pos: Pos) -> Option<Tile> {
        self.tiles.get(pos).copied()
    }
    pub fn set_tile(&mut self, pos: Pos, tile: Tile) {
        if self.start == pos || self.end == pos {
            return;
        }
        self.tiles.set(pos, tile);
    }

    pub fn start(&self) -> Pos {
        self.start
    }
    pub fn set_start(&mut self, pos: Pos) {
        self.set_tile(pos, Tile::None);
        self.start = pos;
    }

    pub fn end(&self) -> Pos {
        self.end
    }
    pub fn set_end(&mut self, pos: Pos) {
        self.set_tile(pos, Tile::None);
        self.end = pos;
    }
}

// description of pathfinding algorithms https://happycoding.io/tutorials/libgdx/pathfinding
pub mod a_star;
pub mod best_first;
pub mod breadth_first;
pub mod depth_first;
pub mod dijkstra;
pub mod distance;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PathFindAlgorithms {
    DepthFirst,
    BreadthFirst,
    Dijkstra,
    AStar,
}
impl PathFindAlgorithms {
    pub fn make_state(&self, grid: Grid) -> Box<dyn PathFindAlgorithm> {
        match self {
            Self::DepthFirst => Box::new(depth_first::DepthFirst::make_state(grid)),
            Self::BreadthFirst => Box::new(BreadthFirst::make_state(grid)),
            Self::Dijkstra => Box::new(dijkstra::Dijkstra::make_state(grid)),
            Self::AStar => Box::new(a_star::AStar::make_state(grid)),
        }
    }
}

pub trait PathFindAlgorithmConstructor {
    fn make_state(grid: Grid) -> Self;
}
pub trait PathFindAlgorithm {
    fn next_step(&mut self) -> Result<Vec<Pos>, PathFindAlgorithmStepResult>;

    fn visited(&self, pos: Pos) -> bool;
    fn visited_list(&self) -> &[Pos];
    fn in_queue(&self, pos: Pos) -> bool;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PathFindAlgorithmStepResult {
    InProgress,
    NotFound,
}
