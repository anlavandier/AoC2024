use std::collections::HashSet;
use std::io::{Result, BufRead, BufReader};
use std::fs::File;

fn main() -> Result<()>{
    let grid = parse("input_test.txt")?;

    let mut score_sum = 0;
    let mut rating_sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                score_sum += trailhead_score((i, j), &grid);
                rating_sum += trailhead_rating((i, j), &grid);
            }
        }
    }

    println!("Score of trailheads {score_sum}");
    println!("Rating of trailheads {rating_sum}");

    Ok(())
}


fn parse(path: &str) -> Result<Vec<Vec<u32>>> {
    let file = BufReader::new(File::open(path)?);

    let mut grid = vec![];

    for line in file.lines() {
        let line = line?;
        let grid_line = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(grid_line);
    }
    return Ok(grid);
}


fn trailhead_score(trailhead: (usize, usize), grid: &Vec<Vec<u32>>) -> usize {
    assert!(grid[trailhead.0][trailhead.1] == 0);
    return travel_along_hiking_trail(grid, trailhead).len();
}


fn travel_along_hiking_trail(grid: &Vec<Vec<u32>>, position: (usize, usize)) -> HashSet<(usize, usize)> {
    let trail_height = grid[position.0][position.1];

    if trail_height == 9 {
        return HashSet::from([position]);
    }

    let mut trail_ends = HashSet::new();


    if position.0 > 0 {
        // Up neighbor
        let up_neigh = (position.0 - 1, position.1);
        if grid[up_neigh.0][up_neigh.1] == trail_height + 1 {
            trail_ends.extend(travel_along_hiking_trail(grid, up_neigh));
        }
    }
    if position.0 < grid.len() - 1 {
        // Down neighbor
        let down_neigh = (position.0 + 1, position.1);
        if grid[down_neigh.0][down_neigh.1] == trail_height + 1 {
            trail_ends.extend(travel_along_hiking_trail(grid, down_neigh));
        }
    }
    if position.1 > 0 {
        // Left neighbor
        let left_neigh = (position.0, position.1 - 1);
        if grid[left_neigh.0][left_neigh.1] == trail_height + 1 {
            trail_ends.extend(travel_along_hiking_trail(grid, left_neigh));
        }
    }
    if position.1 < grid.len() - 1 {
        // Right neighbor
        let right_neigh = (position.0, position.1 + 1);
        if grid[right_neigh.0][right_neigh.1] == trail_height + 1 {
            trail_ends.extend(travel_along_hiking_trail(grid, right_neigh));
        }
    }

    return trail_ends;
}

fn trailhead_rating(trailhead: (usize, usize), grid: &Vec<Vec<u32>>) -> usize {
    assert!(grid[trailhead.0][trailhead.1] == 0);
    return record_hiking_trails(grid, trailhead).len();
}


fn record_hiking_trails(grid: &Vec<Vec<u32>>, position: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let trail_height = grid[position.0][position.1];

    if trail_height == 9 {
        return vec![vec![position]];
    }

    let mut trails_from_there = vec![];

    if position.0 > 0 {
        // Up neighbor
        let up_neigh = (position.0 - 1, position.1);
        if grid[up_neigh.0][up_neigh.1] == trail_height + 1 {
            let trails_from_next = record_hiking_trails(grid, up_neigh);
            trails_from_there.extend(trails_from_next.into_iter().map(|mut v| {
                v.push(position);
                v
            }));
        }
    }
    if position.0 < grid.len() - 1 {
        // Down neighbor
        let down_neigh = (position.0 + 1, position.1);
        if grid[down_neigh.0][down_neigh.1] == trail_height + 1 {
            let trails_from_next = record_hiking_trails(grid, down_neigh);
            trails_from_there.extend(trails_from_next.into_iter().map(|mut v| {
                v.push(position);
                v
            }));        
        }
    }
    if position.1 > 0 {
        // Left neighbor
        let left_neigh = (position.0, position.1 - 1);
        if grid[left_neigh.0][left_neigh.1] == trail_height + 1 {
            let trails_from_next = record_hiking_trails(grid, left_neigh);
            trails_from_there.extend(trails_from_next.into_iter().map(|mut v| {
                v.push(position);
                v
            }));
        }
    }
    if position.1 < grid.len() - 1 {
        // Right neighbor
        let right_neigh = (position.0, position.1 + 1);
        if grid[right_neigh.0][right_neigh.1] == trail_height + 1 {
            let trails_from_next = record_hiking_trails(grid, right_neigh);
            trails_from_there.extend(trails_from_next.into_iter().map(|mut v| {
                v.push(position);
                v
            }));        
        }
    }

    return trails_from_there;
}