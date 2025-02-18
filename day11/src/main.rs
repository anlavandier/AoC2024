use std::io::{BufReader, Read, Result};
use std::fs::File;


fn main() -> Result<()>{
    let mut stones = parse_input("input.txt")?;
    println!("{stones:?}");

    for blink_count in 0..75 {
        println!("{blink_count}");
        stones = blink(&stones);
        //println!("{stones:?}");
    }
    println!("{}", stones.len());
    Ok(())
}





fn parse_input(path: &str) -> Result<Vec<u64>> {
    let mut file = BufReader::new(File::open(path)?);

    let mut line = String::new();
    let _ = file.read_to_string(&mut line)?;

    return Ok(line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect())
}


fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = vec![];

    for s in stones {
        let num_digits= (*s as f64).log10().floor() as u32 + 1;
        if *s == 0 {
            new_stones.push(1);
        }
        else if num_digits % 2 == 0 {
            new_stones.push(*s / 10_u64.pow(num_digits/ 2));
            new_stones.push(*s % 10_u64.pow(num_digits/ 2));
        }
        else {
            new_stones.push(*s * 2024);
        }
    }

    return new_stones
}