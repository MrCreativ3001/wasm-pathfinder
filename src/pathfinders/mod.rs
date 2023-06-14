pub type Unit = u32;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Pos {
    pub x: Unit,
    pub y: Unit,
}
impl Pos {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    None,
    Wall,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    rows: Unit,
    columns: Unit,
    tiles: Vec<Tile>,
    start: Pos,
    end: Pos,
}

impl Grid {
    pub fn new(rows: Unit, columns: Unit, start: Pos, end: Pos) -> Self {
        let tiles = vec![Tile::None; (rows * columns) as usize];
        Self {
            rows,
            columns,
            tiles,
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
        self.tiles[self.index(pos)]
    }
    pub fn tile_opt(&self, pos: Pos) -> Option<Tile> {
        let i = (pos.x + pos.y) as usize;
        if i > self.tiles.len() {
            return None;
        }
        self.tiles.get(self.index(pos)).copied()
    }
    pub fn set_tile(&mut self, pos: Pos, tile: Tile) {
        if self.start == pos || self.end == pos {
            return;
        }
        let i = self.index(pos);
        self.tiles[i] = tile;
    }
    fn index(&self, pos: Pos) -> usize {
        (pos.x + (pos.y * self.columns)) as usize
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

pub mod astar;

pub enum PathFinders {
    AStar,
}

pub trait PathFinder {
    fn find_path(grid: &Grid) -> Vec<Pos>;
}
