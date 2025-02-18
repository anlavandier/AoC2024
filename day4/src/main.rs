use std::collections::HashMap;
use std::io::{BufRead, BufReader, Result};
use std::fs::File;
fn main() -> Result<()>{
    let (grid, width, height) = parse_input("input.txt")?;

    println!("{}, {}", width, height);
    count_christmas(&grid, width, height);
    count_cross_mas(&grid, width, height);
    Ok(())
}


// XMAS can be horizontal, forward or backwards. 
// Vertical
// Diagonal
// Can Be overlapping so every X can be the start of an XMAS. 


#[derive(Debug, PartialEq, Eq)]
pub enum XMAS {
    X,
    M,
    A,
    S,
    Other,
}

fn parse_input(path: &str) -> Result<(HashMap<(i32, i32), XMAS>, i32, i32)> {
    let mut line_num = 0;
    let mut column_num = 0;
    let file = BufReader::new(File::open(path)?);

    let mut christmas_grid = HashMap::new();

    for line in file.lines() {
        let line = line?;
        column_num = 0;
        for char in line.chars() {
            let xmas_charac= match char {
                'X' => {
                    XMAS::X
                }
                'M' => {
                    XMAS::M
                }
                'A' => {
                    XMAS::A
                }
                'S' => {
                    XMAS::S
                }
                _ => {
                    XMAS::Other
                }
            };
            christmas_grid.insert((line_num, column_num), xmas_charac);
            column_num += 1;
        }
        line_num += 1;
    }
    return Ok((christmas_grid, line_num, column_num))
}


fn count_christmas(grid: &HashMap<(i32, i32), XMAS>, width: i32, height: i32) {
    let mut complete_count = 0;

    for (pos, char_type) in grid.iter() {
        if *char_type == XMAS::X {
            let pos_start = (pos.0, pos.1);

            // Eight possibilities starting from this position
            // Forward
            let positions_f = [
                    (pos_start.0 + 1, pos_start.1),  
                    (pos_start.0 + 2, pos_start.1), 
                    (pos_start.0 + 3, pos_start.1)
                ];

            // Backward
            let positions_b = [
                    (pos_start.0 - 1, pos_start.1),  
                    (pos_start.0 - 2, pos_start.1), 
                    (pos_start.0 - 3, pos_start.1)
                ];
            // Down
            let positions_d = [
                    (pos_start.0, pos_start.1 + 1),
                    (pos_start.0, pos_start.1 + 2), 
                    (pos_start.0, pos_start.1 + 3),
                ];
            // Up
            let positions_u = [
                    (pos_start.0, pos_start.1 - 1),
                    (pos_start.0, pos_start.1 - 2), 
                    (pos_start.0, pos_start.1 - 3),
                ];
            // Down Forward    
            let positions_df = [
                    (pos_start.0 + 1, pos_start.1 + 1),
                    (pos_start.0 + 2, pos_start.1 + 2), 
                    (pos_start.0 + 3, pos_start.1 + 3),
                ];
            // Down Backward
            let positions_db = [
                    (pos_start.0 - 1, pos_start.1 + 1),
                    (pos_start.0 - 2, pos_start.1 + 2), 
                    (pos_start.0 - 3, pos_start.1 + 3),
                ];
            // Up Forward
            let positions_uf = [
                    (pos_start.0 + 1, pos_start.1 - 1),
                    (pos_start.0 + 2, pos_start.1 - 2), 
                    (pos_start.0 + 3, pos_start.1 - 3),
                ];
            // Up Backward    
            let positions_ub = [
                    (pos_start.0 - 1, pos_start.1 - 1),
                    (pos_start.0 - 2, pos_start.1 - 2), 
                    (pos_start.0 - 3, pos_start.1 - 3),
                ];

            complete_count += check_for_mas(grid, positions_f, width, height) as i32;
            complete_count += check_for_mas(grid, positions_b, width, height) as i32;
            complete_count += check_for_mas(grid, positions_d, width, height) as i32;
            complete_count += check_for_mas(grid, positions_u, width, height) as i32;
            complete_count += check_for_mas(grid, positions_df, width, height) as i32;
            complete_count += check_for_mas(grid, positions_db, width, height) as i32;
            complete_count += check_for_mas(grid, positions_uf, width, height) as i32;
            complete_count += check_for_mas(grid, positions_ub, width, height) as i32;
        }
    }
    println!("{}", complete_count);
}


fn check_for_mas(grid: &HashMap<(i32, i32), XMAS>, positions: [(i32, i32); 3], width: i32, height: i32) -> bool {
    let mut count = 0;
    for (cx, cy) in positions {
        if cx >= width || cx < 0 || cy >= height || cy < 0 {
            return false
        }
        let approp_c = match count {
            0 => XMAS::M,
            1 => XMAS::A,
            2 => XMAS::S,
            _ => unreachable!(),
        };
        if *grid.get(&(cx, cy)).unwrap() != approp_c {
            return false;
        }
        count += 1;
    }
    return true
}

fn count_cross_mas(grid: &HashMap<(i32, i32), XMAS>, width: i32, height: i32) {
    let mut complete_count = 0;

    for (pos, char_type) in grid.iter() {
        if *char_type == XMAS::M {
            let pos_start = (pos.0, pos.1);

            // 2 possibilities starting from this start
            // M . M
            // . A .
            // S . S
            let positions_mmas = [
                    (pos_start.0 + 2, pos_start.1, XMAS::M),
                    (pos_start.0 + 1, pos_start.1 + 1, XMAS::A),
                    (pos_start.0, pos_start.1 + 2, XMAS::S),
                    (pos_start.0 + 2, pos_start.1 + 2, XMAS::S),
                ];

            // M . S
            // . A .
            // M . S
            let positions_msam = [
                (pos_start.0 + 2, pos_start.1, XMAS::S),
                (pos_start.0 + 1, pos_start.1 + 1, XMAS::A),
                (pos_start.0, pos_start.1 + 2, XMAS::M),
                (pos_start.0 + 2, pos_start.1 + 2, XMAS::S),
                ];

            complete_count += check_pattern(grid, positions_mmas, width, height) as i32;
            complete_count += check_pattern(grid, positions_msam, width, height) as i32;
        }
        if *char_type == XMAS::S {
            let pos_start = (pos.0, pos.1);

            // 2 possibilities starting from this start
            // S . M
            // . A .
            // S . M
            let positions_smas = [
                    (pos_start.0 + 2, pos_start.1, XMAS::M),
                    (pos_start.0 + 1, pos_start.1 + 1, XMAS::A),
                    (pos_start.0, pos_start.1 + 2, XMAS::S),
                    (pos_start.0 + 2, pos_start.1 + 2, XMAS::M),
                ];

            // S . S
            // . A .
            // M . M
            let positions_ssam = [
                (pos_start.0 + 2, pos_start.1, XMAS::S),
                (pos_start.0 + 1, pos_start.1 + 1, XMAS::A),
                (pos_start.0, pos_start.1 + 2, XMAS::M),
                (pos_start.0 + 2, pos_start.1 + 2, XMAS::M),
                ];

            complete_count += check_pattern(grid, positions_smas, width, height) as i32;
            complete_count += check_pattern(grid, positions_ssam, width, height) as i32;
        }
    }
    println!("{}", complete_count);
}

fn check_pattern(grid: &HashMap<(i32, i32), XMAS>, positions: [(i32, i32, XMAS); 4], width: i32, height: i32) -> bool {
    for (cx, cy, letter) in positions {
        if cx >= width || cx < 0 || cy >= height || cy < 0 {
            return false
        }
        if *grid.get(&(cx, cy)).unwrap() != letter {
            return false;
        }
    }
    return true
}

