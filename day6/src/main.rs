use std::collections::HashSet;
use std::io::{Result, BufRead, BufReader};
use std::fs::File;

fn main() -> Result<()> {
    let (mut grid, start_pos) = parse_input("input_test.txt")?;

    let mut guard = (start_pos, North);
    let mut visited_pos = HashSet::new();

    while guard.1 != Out {
        visited_pos.insert(guard.0);
        guard = advance(&grid, guard);
    }

    println!(" Number of visited positions {}", visited_pos.len());

    let mut loop_starts = vec![];
    for pos1 in 0..grid.len() {//in visited_pos {
        for pos2 in 0..grid[0].len() {
            let pos = (pos1, pos2);
            if pos == start_pos { continue; }
            if grid[pos.0][pos.1] { continue; }
            // Add the Wall
            grid[pos.0][pos.1] = true;
            // Loop = back to the initial State
            guard = (start_pos, North);
            let mut visited_states = HashSet::new();
            // println!("Testing with an obstacle on {:?}", pos);
            loop {
                guard = advance(&grid, guard);

                if guard.1 == Out { break; }
                if !visited_states.insert(guard) { loop_starts.push(pos); break;}
            }
            // Remove the Wall
            grid[pos.0][pos.1] = false;
        }
    }
    println!("{} ways of introducing loops", loop_starts.len());
    //println!("Obstacle positions:\n{:?}", loop_starts);

    Ok(()) 
}

fn parse_input(path: &str) -> Result<(Vec<Vec<bool>>, (usize, usize))> {
    let file = BufReader::new(File::open(path)?);

    let mut grid = vec![];
    let mut start_pos = (0, 0);

    let mut x_pos = 0;

    for line in file.lines() {
        let mut grid_line = vec![];
        let mut y_pos = 0;
        let line = line?;
        for char in line.chars() {
            match char {
                '^' => { 
                    start_pos = (x_pos, y_pos);
                    grid_line.push(false);
                },
                '#' => {
                    grid_line.push(true);
                },
                '.' => {
                    grid_line.push(false);
                },
                _ => unreachable!()
            }
            y_pos += 1;
        }
        grid.push(grid_line);
        x_pos += 1;
    }

    return Ok((grid, start_pos));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
    Out,
}
use Direction::*;

fn advance(grid: &Vec<Vec<bool>>, guard: ((usize, usize), Direction)) -> ((usize, usize), Direction) {
    let (start_pos, dir) = guard;
    return match dir {
        North => {
            if start_pos.0 == 0 {((0, 0), Out)}
            else {
                let maybe_new_pos = (start_pos.0 - 1, start_pos.1);
                let wall = grid.get(maybe_new_pos.0)
                    .map(|line| line.get(maybe_new_pos.1)).flatten();
                if let Some(w) = wall {
                    if *w { ((start_pos.0, start_pos.1 + 1), East) }
                    else { (maybe_new_pos, dir) }
                }
                else { ((0, 0), Out) }
            }
        }
        South => {
            let maybe_new_pos = (start_pos.0 + 1, start_pos.1);
            let wall = grid.get(maybe_new_pos.0)
                .map(|line| line.get(maybe_new_pos.1)).flatten();
            if let Some(w) = wall {
                if *w { ((start_pos.0, start_pos.1 - 1), West) }
                else { (maybe_new_pos, dir) }
            }
            else { ((0, 0), Out) }
        }
        East => {
            let maybe_new_pos = (start_pos.0, start_pos.1 + 1);
            let wall = grid.get(maybe_new_pos.0)
                .map(|line| line.get(maybe_new_pos.1)).flatten();
            if let Some(w) = wall {
                if *w { ((start_pos.0 + 1, start_pos.1), South) }
                else { (maybe_new_pos, dir) }
            }
            else { ((0, 0), Out) }
        }
        West => {
            if start_pos.1 == 0 {((0, 0), Out)} 
            else {
                let maybe_new_pos = (start_pos.0, start_pos.1 - 1);
                let wall = grid.get(maybe_new_pos.0)
                    .map(|line| line.get(maybe_new_pos.1)).flatten();
                if let Some(w) = wall {
                    if *w { ((start_pos.0 - 1, start_pos.1), North) }
                    else { (maybe_new_pos, dir) }
                }
                else { ((0, 0), Out) }
            }
        }
        Out => unreachable!()
    };
}