use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Result};
use std::fs::File;
use std::ops::{Add, Sub, AddAssign, SubAssign};


fn antinodes_in_grid(p_1: Point, p_2: Point, width: isize, height: isize) -> Vec<Point> {
    let mut antinodes = vec![p_1, p_2];


    let dir = p_2 - p_1;
    let mut new_antinode = p_2 + dir;

    let in_grid = |p: Point| {
        p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height
    };

    while in_grid(new_antinode) {
        antinodes.push(new_antinode);

        new_antinode += dir;
    }
    return antinodes;
}


fn main() -> Result<()>{
    let (antennas, width, height) = parse_input("input.txt")?;

    let mut antinode_positions = HashSet::new();
    for (_antenna_type, antenna_positions) in antennas.iter() {
        for antenna_a in antenna_positions.iter() {
            for antenna_b in antenna_positions.iter() {
                if antenna_a == antenna_b { continue; }
                antinode_positions.extend(antinodes_in_grid(*antenna_a, *antenna_b, width, height));
            }
        }
    }
    println!("Number of unique antinode positions: {}", antinode_positions.len());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

fn parse_input(path: &str) -> Result<(HashMap<char, Vec<Point>>, isize, isize)> {
    let file = BufReader::new(File::open(path)?);

    let mut antennas = HashMap::new();

    let mut line_index = 0;
    let mut column_index = 0;
    for line in file.lines() {
        let line = line?;

        column_index = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                c => {
                    antennas.entry(c)
                        .and_modify(|v: &mut Vec<Point>| v.push(Point(line_index, column_index)))
                        .or_insert(vec![Point(line_index, column_index)]);
                }
            }
            column_index += 1
        }
        line_index += 1
    }

    return Ok((antennas, line_index, column_index));
}
