use std::fs::File;
use std::io::{Result, BufRead, BufReader};


fn valid_levels<'a, T: Iterator<Item = &'a i32>>(mut report: T, length: usize) -> bool {
    match length {
        0..=2 => {
            true
        }
        _ => {
            let mut prev = report.next().unwrap();
            let mut cur = report.next().unwrap();
            let mut diff = prev - cur;
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            let decr = diff > 0;
            prev = cur;
            let mut valid = true;
            for _ in 2..length {
                cur = report.next().unwrap();
                diff = prev - cur;
                if diff.abs() < 1 || diff.abs() > 3 { valid = false; break; }
                if (diff > 0) != decr { valid = false; break; }
                prev = cur;
            }
            return valid;
        }
    }
}


fn main() -> Result<()>{
    let input = BufReader::new(File::open("input.txt")?);

    let mut num_safe = 0;
    let mut num_safe_with_one_error = 0;

    for line in input.lines() {
        let line = line?;
        let report: Vec<i32> = line.split(' ')
                          .map(|x: &str| {x.parse::<i32>().unwrap()})
                          .collect::<Vec<i32>>();

        // Safe report requirements:
        // 1 - Levels are all decreasing or increasing
        // 1 - abs diffs between levels is in [1, 3]
        // Check validity without changing
        let is_safe = valid_levels(report.iter(), report.len());
        // First, check all possible changes until a valid one is found
        if is_safe { num_safe += 1; }
        else {
            for i in 0..report.len() {
                let my_partial_iter  = report[..i].iter().chain(report[i+1..].iter());
                let is_safe_one_error = valid_levels(my_partial_iter, report.len() - 1);
                if is_safe_one_error { num_safe_with_one_error += 1; break; }
            }
        }

    }

    println!("{num_safe} Safe Levels");
    println!("{num_safe_with_one_error} Safe with one error");
    let total = num_safe + num_safe_with_one_error;
    println!("Total :{total}");
    Ok(())
}
