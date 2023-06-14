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
    rows: usize,
    columns: usize,
    flattened: Vec<T>,
}

impl<T> Vec2d<T> {
    pub fn new(rows: usize, columns: usize, value: T) -> Self
    where
        T: Clone,
    {
        let flattened = vec![value; rows * columns];
        Self {
            rows,
            columns,
            flattened,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn columns(&self) -> usize {
        self.columns
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
        if x >= self.columns || y >= self.rows {
            return None;
        }
        Some(y * self.columns + x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    rows: Unit,
    columns: Unit,
    tiles: Vec2d<Tile>,
    start: Pos,
    end: Pos,
}

impl Grid {
    pub fn new(rows: Unit, columns: Unit, start: Pos, end: Pos) -> Self {
        Self {
            rows,
            columns,
            tiles: Vec2d::new(rows as usize, columns as usize, Tile::None),
            start,
            end,
        }
    }

    pub fn rows(&self) -> Unit {
        self.rows
    }
    pub fn columns(&self) -> Unit {
        self.columns
    }

    pub fn tile(&self, pos: Pos) -> Tile {
        *self.tiles.get(pos).expect("invalid position")
    }
    pub fn tile_opt(&self, pos: Pos) -> Option<Tile> {
        self.tiles.get(pos).map(|tile| *tile)
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
        self.start = pos;
        self.set_tile(pos, Tile::None);
    }

    pub fn end(&self) -> Pos {
        self.end
    }
    pub fn set_end(&mut self, pos: Pos) {
        self.end = pos;
        self.set_tile(pos, Tile::None);
    }
}

// description of pathfinding algorithms https://happycoding.io/tutorials/libgdx/pathfinding#a
pub mod astar;
pub mod breadth_first;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PathFinders {
    BreadthFirst,
    AStar,
}

impl PathFinders {
    pub fn find_path(&self, grid: &Grid) -> Option<Vec<Pos>> {
        match self {
            Self::BreadthFirst => breadth_first::BreadthFirst::find_path(grid),
            Self::AStar => astar::AStar::find_path(grid),
        }
    }
}

pub trait PathFinder {
    fn find_path(grid: &Grid) -> Option<Vec<Pos>>;
}
