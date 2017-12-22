// --- Day 19: A Series of Tubes ---

// Somehow, a network packet got lost and ended up here. It's trying to follow a routing diagram (your puzzle input), but it's confused about where to go.

// Its starting point is just off the top of the diagram. Lines (drawn with |, -, and +) show the path it needs to take, starting by going down onto the only line connected to the top of the diagram. It needs to follow this path until it reaches the end (located somewhere within the diagram) and stop there.

// Sometimes, the lines cross over each other; in these cases, it needs to continue going the same direction, and only turn left or right when there's no other option. In addition, someone has left letters on the line; these also don't change its direction, but it can use them to keep track of where it's been. For example:

//      |          
//      |  +--+    
//      A  |  C    
//  F---|----E|--+ 
//      |  |  |  D 
//      +B-+  +--+ 

// Given this diagram, the packet needs to take the following path:

//     Starting at the only line touching the top of the diagram, it must go down, pass through A, and continue onward to the first +.
//     Travel right, up, and right, passing through B in the process.
//     Continue down (collecting C), right, and up (collecting D).
//     Finally, go all the way left through E and stopping at F.

// Following the path to the end, the letters it sees on its path are ABCDEF.

// The little packet looks up at you, hoping you can help it find the way. What letters will it see (in the order it would see them) if it follows the path? (The routing diagram is very wide; make sure you view it without line wrapping.)
extern crate ndarray;

use self::ndarray::{Array2, Axis};
use std::convert::TryInto;

#[derive(Clone, Copy, Debug)]
enum Direction { Up, Left, Down, Right }

impl Direction {
    fn left(self) -> Self {
        use self::Direction::*;

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn right(self) -> Self {
        use self::Direction::*;

        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Straight,
    Corner,
    Letter(char),
}

impl Tile {
    fn new(symbol: char) -> Option<Self> {
        use self::Tile::*;

        match symbol {
            ' ' => None,
            '-' | '|' => Some(Straight),
            '+' => Some(Corner),
            l @ 'A'...'Z' => Some(Letter(l)),
            _ => panic!(),
        }
    }
}

const X: Axis = Axis(1);
const Y: Axis = Axis(0);

struct Map<'a> {
    array: &'a Array2<Option<Tile>>,
    pos: [usize; 2],
    dir: Direction,
    tile: Tile,
}

impl<'a> Map<'a> {
    fn new(array: &'a Array2<Option<Tile>>) -> Self {
        let x_pos = array.iter().position(|tile| tile.is_some()).unwrap();
        let pos = [0, x_pos];
        let tile = array[pos].unwrap();

        Map {
            array,
            pos,
            dir: Direction::Down,
            tile,
        }
    }

    fn step(self, letters: &mut Vec<char>) -> Option<Self> {
        use self::Tile::*;

        if let Letter(l) = self.tile {
            letters.push(l);
        }

        match self.tile {
            Straight | Letter(_) => {
                self.move_adjacent(self.dir)
            },
            Corner => {
                self.move_adjacent(self.dir.left())
                    .or(self.move_adjacent(self.dir.right()))
            }
        }
    }

    fn move_adjacent(&self, dir: Direction) -> Option<Self> {
        let pos = Map::adjacent_coords(self.pos, dir)?;
        let tile = Map::get_tile(self.array, pos)?;

        Some(Map {
            array: self.array,
            pos,
            dir,
            tile,
        })
    }

    fn get_tile(array: &Array2<Option<Tile>>, coords: [usize; 2]) -> Option<Tile> {
        let [y, x] = coords;

        if y >= array.len_of(Y) || x >= array.len_of(X) {
            None
        } else {
            array[[y, x]]
        }
    }

    fn adjacent_coords(coords: [usize; 2], dir: Direction) -> Option<[usize; 2]> {
        use self::Direction::*;

        let [y, x] = coords;

        let [offs_y, offs_x] = match dir {
            Up => [-1, 0],
            Left => [0, -1],
            Down => [1, 0],
            Right => [0, 1],
        };

        let [yi, xi] = [y as isize + offs_y, x as isize + offs_x];

        Some([yi.try_into().ok()?, xi.try_into().ok()?])
    }
}

pub fn part1(input: &str) -> String {
    let rows = || input.split('\n').map(|row| row.chars().map(|c| Tile::new(c)));
    let y = rows().count();
    let x = rows().next().unwrap().count();

    let tiles = rows().flat_map(|tile| tile).collect();
    let array = Array2::from_shape_vec((y, x), tiles).unwrap();

    let mut letters = Vec::new();

    let mut map = Map::new(&array);

    while let Some(next_map) = map.step(&mut letters) {
        map = next_map;
    }

    letters.into_iter().collect()
}