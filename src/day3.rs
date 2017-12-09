// --- Day 3: Spiral Memory ---

// You come across an experimental new kind of memory stored on an infinite two-dimensional grid.

// Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and then counting up while spiraling outward. For example, the first few squares are allocated like this:

// 17  16  15  14  13
// 18   5   4   3  12
// 19   6   1   2  11
// 20   7   8   9  10
// 21  22  23---> ...

// While this is very space-efficient (no squares are skipped), requested data must be carried back to square 1 (the location of the only access port for this memory system) by programs that can only move up, down, left, or right. They always take the shortest path: the Manhattan Distance between the location of the data and square 1.

// For example:

//     Data from square 1 is carried 0 steps, since it's at the access port.
//     Data from square 12 is carried 3 steps, such as: down, left, left.
//     Data from square 23 is carried only 2 steps: up twice.
//     Data from square 1024 must be carried 31 steps.

// How many steps are required to carry the data from the square identified in your puzzle input all the way to the access port?
use std::collections::HashMap;
use self::spiral::Spiral;


pub fn part1(input: &str) -> String {
    let val: i32 = input.parse().unwrap();

    if val < 1 { panic!("Input value must be positive"); }

    // Avoids dividing by zero on a "zero-length side" later on
    if val == 1 { return 0.to_string(); }

    // Get root of next odd square >= val
    let odd_root = (1..).step_by(2).find(|r| r * r >= val).unwrap();
    
    // Coords of odd square from origin
    let (odd_square_x, odd_square_y) = (odd_root / 2, -odd_root / 2);

    // Offset of `val` from the odd square's position
    let (val_offs_x, val_offs_y) = {
        // Left, up, right, down
        let anticlockwise_dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let odd_square_diff = (odd_root * odd_root) - val;
        let side_len = odd_root - 1;
        let full_sides = (odd_square_diff / side_len) as usize;
        let rem = odd_square_diff % side_len;

        let (full_offs_x, full_offs_y) = anticlockwise_dirs.iter()
            .take(full_sides)
            .fold((0, 0), |(acc_x, acc_y), &(x, y)| {
                (acc_x + x * side_len, acc_y + y * side_len)
            });

        let (rem_dir_x, rem_dir_y) = anticlockwise_dirs[full_sides];

        (full_offs_x + rem_dir_x * rem, full_offs_y + rem_dir_y * rem)
    };

    let ans = (odd_square_x + val_offs_x).abs() + (odd_square_y + val_offs_y).abs();

    ans.to_string()
}

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
pub fn part2(input: &str) -> String {
    let target_val = input.parse().unwrap();
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
            return cell_val.to_string();
        }

        cells.insert((x, y), cell_val);
    }

    unreachable!();
}

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