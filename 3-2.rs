// --- Part Two ---

// As a stress test on the system, the programs here clear the grid and then store the value 1 in square 1. Then, in the same allocation order as shown above, they store the sum of the values in all adjacent squares, including diagonals.

// So, the first few squares' values are chosen as follows:

//     Square 1 starts with the value 1.
//     Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
//     Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
//     Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
//     Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.

// Once a square is written, its value does not change. Therefore, the first few squares would receive the following values:

// 147  142  133  122   59
// 304    5    4    2   57
// 330   10    1    1   54
// 351   11   23   25   26
// 362  747  806--->   ...

// What is the first value written that is larger than your puzzle input?

// Your puzzle input is still 277678.

#![feature(inclusive_range_syntax)]

mod spiral {
    use std::iter::Iterator;
    use std::ops::Range;
    use std::mem;

    use self::Dir::{Up, Down, Left, Right};

    enum Dir { Up, Down, Left, Right }

    pub struct Spiral { line: Range<usize>, coords: (i32, i32), dir: Dir }

    impl Spiral {
        pub fn new() -> Self {
            Self { line: 0..1, coords: (0, 0), dir: Dir::Right }
        }

        fn turn(&mut self) {
            let dir = match self.dir {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up
            };

            let line = match self.dir {
                Right | Left => 0..self.line.end,
                Up | Down => 0..(self.line.end + 1)
            };

            *self = Self { line, dir, ..*self };
        }
    }

    impl Iterator for Spiral {
        type Item = (i32, i32);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(_) = self.line.next() {
                let offset = match self.dir {
                    Up => (0, 1),
                    Down => (0, -1),
                    Left => (-1, 0),
                    Right => (1, 0),
                };

                let mut coords = (self.coords.0 + offset.0, self.coords.1 + offset.1);
                mem::swap(&mut self.coords, &mut coords);

                Some(coords)
            } else {
                self.turn();
                self.next()
            }
        }
    }
}

use std::collections::HashMap;
use std::env;
use self::spiral::Spiral;

fn solve(target_val: i32) -> i32 {
    let mut spiral = Spiral::new();
    let mut cells = HashMap::new();
    cells.insert(spiral.next().unwrap(), 1);

    for (x, y) in spiral {
        let mut cell_val = 0;

        for x_offs in -1..=1 {
            for y_offs in -1..=1 {
                if let Some(surr_val) = cells.get(&(x + x_offs, y + y_offs)) {
                    cell_val += surr_val;
                }
            }
        }

        if cell_val > target_val {
            return cell_val;
        }

        cells.insert((x, y), cell_val);
    }

    unreachable!();
}

fn main() {
    let val: i32 = env::args().nth(1).unwrap().parse().unwrap();

    let ans = solve(val);

    println!("{}", ans);
}