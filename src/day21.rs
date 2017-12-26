// --- Day 21: Fractal Art ---

// You find a program trying to generate some art. It uses a strange process that involves repeatedly enhancing the detail of an image through a set of rules.

// The image consists of a two-dimensional square grid of pixels that are either on (#) or off (.). The program always begins with this pattern:

// .#.
// ..#
// ###

// Because the pattern is both 3 pixels wide and 3 pixels tall, it is said to have a size of 3.

// Then, the program repeats the following process:

//     If the size is evenly divisible by 2, break the pixels up into 2x2 squares, and convert each 2x2 square into a 3x3 square by following the corresponding enhancement rule.
//     Otherwise, the size is evenly divisible by 3; break the pixels up into 3x3 squares, and convert each 3x3 square into a 4x4 square by following the corresponding enhancement rule.

// Because each square of pixels is replaced by a larger one, the image gains pixels and so its size increases.

// The artist's book of enhancement rules is nearby (your puzzle input); however, it seems to be missing rules. The artist explains that sometimes, one must rotate or flip the input pattern to find a match. (Never rotate or flip the output pattern, though.) Each pattern is written concisely: rows are listed as single units, ordered top-down, and separated by slashes. For example, the following rules correspond to the adjacent patterns:

// ../.#  =  ..
//           .#

//                 .#.
// .#./..#/###  =  ..#
//                 ###

//                         #..#
// #..#/..../#..#/.##.  =  ....
//                         #..#
//                         .##.

// When searching for a rule to use, rotate and flip the pattern as necessary. For example, all of the following patterns match the same rule:

// .#.   .#.   #..   ###
// ..#   #..   #.#   ..#
// ###   ###   ##.   .#.

// Suppose the book contained the following two rules:

// ../.# => ##./#../...
// .#./..#/### => #..#/..../..../#..#

// As before, the program begins with this pattern:

// .#.
// ..#
// ###

// The size of the grid (3) is not divisible by 2, but it is divisible by 3. It divides evenly into a single square; the square matches the second rule, which produces:

// #..#
// ....
// ....
// #..#

// The size of this enhanced grid (4) is evenly divisible by 2, so that rule is used. It divides evenly into four squares:

// #.|.#
// ..|..
// --+--
// ..|..
// #.|.#

// Each of these squares matches the same rule (../.# => ##./#../...), three of which require some flipping and rotation to line up with the rule. The output for the rule is the same in all four cases:

// ##.|##.
// #..|#..
// ...|...
// ---+---
// ##.|##.
// #..|#..
// ...|...

// Finally, the squares are joined into a new grid:

// ##.##.
// #..#..
// ......
// ##.##.
// #..#..
// ......

// Thus, after 2 iterations, the grid contains 12 pixels that are on.

// How many pixels stay on after 5 iterations?
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

struct Image {
    tiles: Vec<Tile>,
    transforms: HashMap<Tile, Tile>
}

impl Image {
    fn new(initial_tile: &str, rules: &str) -> Self {
        Image {
            tiles: vec![initial_tile.parse().unwrap()],
            transforms: Image::parse_transforms(rules)
        }
    }

    fn transform(tile: Tile, transforms: &HashMap<Tile, Tile>) -> Vec<Tile> {
        use self::Tile::*;

        match tile {
            Size2(_) | Size3(_) => vec![transforms[&tile]],
            Size4(v) => {
                [
                    Size2([v[0], v[1], v[4], v[5]]),
                    Size2([v[2], v[3], v[6], v[7]]),
                    Size2([v[8], v[9], v[12], v[13]]),
                    Size2([v[10], v[11], v[14], v[15]]),
                ].into_iter().map(|tile| transforms[tile]).collect()
            }
        }
    }

    fn parse_transforms(rules: &str) -> HashMap<Tile, Tile> {
        let mut transforms = HashMap::new();

        for line in rules.split('\n') {
            let mut parts = line.split(" => ");
            let from_tile: Tile = parts.next().unwrap().parse().unwrap();
            let to_tile: Tile = parts.next().unwrap().parse().unwrap();

            for from_variation in from_tile.variations() {
                transforms.insert(from_variation, to_tile);
            }
        }

        transforms
    }

    fn enhance(self) -> Self {
        let Image { tiles, transforms } = self;
        Image {
            tiles: tiles.into_iter()
                .flat_map(|tile| Image::transform(tile, &transforms))
                .collect(),
            transforms
        }
    }

    fn count_ones(&self) -> u32 {
        self.tiles.iter().map(|tile| tile.count_ones()).sum()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Size2([bool; 4]),
    Size3([bool; 9]),
    Size4([bool; 16])
}

impl Tile {
    fn new(vals: &[bool]) -> Self {
        use self::Tile::*;

        match vals.len() {
            4 => Size2(*array_ref![vals, 0, 4]),
            9 => Size3(*array_ref![vals, 0, 9]),
            16 => Size4(*array_ref![vals, 0, 16]),
            _ => panic!()
        }
    }

    fn vals(&self) -> &[bool] {
        use self::Tile::*;

        match *self {
            Size2(ref vals) => vals,
            Size3(ref vals) => vals,
            Size4(ref vals) => vals,
        }
    }

    fn flip_v(self) -> Self {
        use self::Tile::*;

        match self {
            Size2(vals) => Size2([
                vals[2], vals[3],
                vals[0], vals[1]
            ]),
            Size3(vals) => Size3([
                vals[6], vals[7], vals[8],
                vals[3], vals[4], vals[5],
                vals[0], vals[1], vals[2],
            ]),
            Size4(vals) => Size4([
                vals[12], vals[13], vals[14], vals[15],
                vals[8], vals[9], vals[10], vals[11],
                vals[4], vals[5], vals[6], vals[7],
                vals[0], vals[1], vals[2], vals[3],
            ])
        }
    }

    fn flip_h(self) -> Self {
        use self::Tile::*;

        match self {
            Size2(vals) => Size2([
                vals[1], vals[0],
                vals[3], vals[2]
            ]),
            Size3(vals) => Size3([
                vals[2], vals[1], vals[0],
                vals[5], vals[4], vals[3],
                vals[8], vals[7], vals[6],
            ]),
            Size4(vals) => Size4([
                vals[3], vals[2], vals[1], vals[0],
                vals[7], vals[6], vals[5], vals[4],
                vals[11], vals[10], vals[9], vals[8],
                vals[15], vals[14], vals[13], vals[12],
            ])
        }
    }

    fn rotate_90(self) -> Self {
        use self::Tile::*;

        match self {
            Size2(vals) => Size2([
                vals[2], vals[0],
                vals[3], vals[1]
            ]),
            Size3(vals) => Size3([
                vals[6], vals[3], vals[0],
                vals[7], vals[4], vals[1],
                vals[8], vals[5], vals[2],
            ]),
            Size4(vals) => Size4([
                vals[12], vals[8], vals[4], vals[0],
                vals[13], vals[9], vals[5], vals[1],
                vals[14], vals[10], vals[6], vals[2],
                vals[15], vals[11], vals[7], vals[3],
            ])
        }
    }

    fn variations(self) -> Vec<Self> {
        let mut variations = Vec::new();

        for &flip_h in &[self, self.flip_h()] {
            for &flip_v in &[flip_h, flip_h.flip_v()] {
                variations.push(flip_v);
                variations.push(flip_v.rotate_90());
            }
        }

        variations
    }

    fn count_ones(self) -> u32 {
        self.vals().iter().map(|&v| if v { 1 } else { 0 }).sum()
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<bool> = s.chars().filter_map(|c| {
            match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None
            }
        }).collect();

        Ok(Tile::new(vals.as_slice()))
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr: String = self.vals().iter()
            .map(|&v| if v { '#' } else { '.' })
            .collect();

        write!(f, "{}", repr)
    }
}

pub fn part1(input: &str) -> String {
    let mut image = Image::new(".#./..#/###", input);

    for _ in 0..5 {
        image = image.enhance();
    }

    let ans = image.count_ones();

    ans.to_string()
}