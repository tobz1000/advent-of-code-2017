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

// Your puzzle input is 277678.

#![feature(iterator_step_by)]

use std::env;


fn solve(val: i32) -> i32 {
    if val < 1 { panic!("Input value must be positive"); }

    // Avoids dividing by zero on a "zero-length side" later on
    if val == 1 { return 0; }

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

    (odd_square_x + val_offs_x).abs() + (odd_square_y + val_offs_y).abs()
}

fn main() {
    let val: i32 = env::args().nth(1).unwrap().parse().unwrap();

    let ans = solve(val);

	println!("{}", ans);
}