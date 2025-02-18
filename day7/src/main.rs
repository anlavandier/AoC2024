use std::fs::File;
use std::io::{Result, BufRead, BufReader};


enum Ops {
    Plus,
    Time,
    Concat,
}

use Ops::*;

fn calculate_left_to_right(
    excepted_result: u64, opnd1: u64, remaining_opnds: &[u64], operation: Ops
) -> Option<Vec<Ops>> {
    if remaining_opnds.is_empty() { return (opnd1 == excepted_result).then(|| vec![]);}
    let (opnd2, remaining) = remaining_opnds.split_at(1);
    
    let new_opnd1 = match operation {
        Plus => {
            opnd1 + opnd2[0]
        }
        Time => {
            opnd1 * opnd2[0]
        },
        Concat => {
            let mut new_opnd1_as_str = opnd1.to_string();
            new_opnd1_as_str.push_str(&opnd2[0].to_string());
            new_opnd1_as_str.parse::<u64>().unwrap()
        }
    };
    // println!("{}, {:?}", new_opnd1, remaining_opnds);
    if new_opnd1 > excepted_result { return  None; }
    let with_plus = calculate_left_to_right(
        excepted_result, 
        new_opnd1, 
        remaining, 
        Plus
        ).map(|mut ops_vec| {ops_vec.insert(0, Plus); ops_vec});
    if with_plus.is_some() { return with_plus; }
    let with_time = calculate_left_to_right(
        excepted_result, 
        new_opnd1, 
        remaining, 
        Time
        ).map(|mut ops_vec| {ops_vec.insert(0, Time); ops_vec});
    if with_time.is_some() { return with_time; }
    let with_concat = calculate_left_to_right(
        excepted_result, 
        new_opnd1, 
        remaining, 
        Concat
        ).map(|mut ops_vec| {ops_vec.insert(0, Concat); ops_vec});
    if with_concat.is_some() { return with_concat; }
    return None; 
}


fn main() -> Result<()>{
    let operations = parse_input("input.txt")?;

    let mut sum_valid = 0;

    //calculate_left_to_right(190, 0, &[10, 19], Plus);
    // return Ok(());

    for (result, opnds) in operations {
        // println!("{}: {:?}", result, opnds);

        let ops_list = calculate_left_to_right(result, 0, &opnds, Plus);
        if ops_list.is_some() { sum_valid += result; }
    }

    println!("{}", sum_valid);

    Ok(())
}

fn parse_input(path: &str) -> Result<Vec<(u64, Vec<u64>)>> {
    let file = BufReader::new(File::open(path)?);

    let mut operations = vec![];

    for line in file.lines() {
        let line = line?;
        let mut split_line = line.split(':');
        let result = split_line.next().map(|n| n.parse::<u64>().unwrap()).unwrap();
        let opnds = split_line.next()
            .map(|s| 
                s.split(' ').into_iter().filter_map(|n| n.parse::<u64>().ok()).collect::<Vec<_>>()
            ).unwrap();   
        operations.push((result, opnds));
    }
    return Ok(operations);
}