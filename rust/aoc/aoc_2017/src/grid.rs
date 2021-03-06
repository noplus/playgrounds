use std::collections::HashMap;
use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;

#[derive(Debug)]
pub struct Grid<T> {
    pub cells: HashMap<(isize, isize), T>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    E,
    N,
    W,
    S,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self { cells: HashMap::new() }
    }

    pub fn from_maze_matrix<Matrix, Row>(matrix: Matrix, is_wall: fn(&T) -> bool) -> Self
        where Matrix: IntoIterator<Item=Row>,
              Row: IntoIterator<Item=T>, {
        let mut s = Self::new();
        for (y, row) in matrix.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate().filter(|(_, t)| !is_wall(t)) {
                s.insert(x as isize, y as isize, cell);
            }
        }
        s
    }

    pub fn get_maze_graph(&self) -> UnGraphMap<(isize, isize), ()> {
        let mut graph = UnGraphMap::new();
        for &(x1, y1) in self.cells.keys() {
            graph.add_node((x1, y1));
            for (x2, y2, _) in self.adjacent_coords(x1, y1) {
                graph.add_edge((x1, y1), (x2, y2), ());
            }
        }
        graph
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.cells.get(&(x, y))
    }

    pub fn insert(&mut self, x: isize, y: isize, v: T) -> Option<T> {
        self.cells.insert((x, y), v)
    }

    pub fn neighbors(&self, x: isize, y: isize) -> Vec<&T> {
        let deltas = [-1, 0, 1];
        deltas.iter().cartesian_product(deltas.iter())
            .filter(|(&i, &j)| (i, j) != (0, 0))
            .flat_map(|(&i, &j)| self.get(x + i, y + j))
            .collect()
    }

    pub fn adjacent_coords(&self, x: isize, y: isize) -> Vec<(isize, isize, Direction)> {
        // like neighbors but doesn't count diagonals
        let directions = [Direction::E, Direction::N, Direction::W, Direction::S];
        let offsets = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        offsets.iter().zip(directions.iter())
            .map(|((i, j), d)| (x + i, y + j, *d))
            .filter(|&(x, y, _)| self.cells.contains_key(&(x, y)))
            .collect()
    }

    pub fn get_adjacent_coord(&self, x: isize, y: isize, d: Direction) -> (isize, isize) {
        match d {
            Direction::E => (x + 1, y),
            Direction::N => (x, y - 1),
            Direction::W => (x - 1, y),
            Direction::S => (x, y + 1),
        }
    }
}

impl Grid<char> {
//    pub fn print(&self) {
//        let (mut minx, mut maxx, mut miny, mut maxy) = (0_f32, 0_f32, 0_f32, 0_f32);
//        for &(x, y) in self.cells.keys() {
//            minx = minx.min(x as f32);
//            miny = miny.min(y as f32);
//            maxx = maxx.max(x as f32);
//            maxy = maxy.max(y as f32);
//        }
//        for y in miny as isize..=maxy as isize {
//            for x in minx as isize..=maxx as isize {
//                print!("{}", self.get(x, y).unwrap_or(&' '));
//            }
//            println!();
//        }
//    }
}

impl Direction {
    pub fn turn_left(d: Direction) -> Direction {
        match d {
            Direction::E => Direction::N,
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
        }
    }
    pub fn turn_right(d: Direction) -> Direction {
        match d {
            Direction::E => Direction::S,
            Direction::N => Direction::E,
            Direction::W => Direction::N,
            Direction::S => Direction::W,
        }
    }
    pub fn reverse(d: Direction) -> Direction {
        match d {
            Direction::E => Direction::W,
            Direction::N => Direction::S,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
        }
    }
}