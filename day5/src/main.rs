use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Result};
use std::fs::File;


fn main() -> Result<()>{
    let (conditions, updates) = parse_file("input.txt")?;

    let (valid_updates, corrected_updates) = 
        find_valid_updates_and_correct_the_rest(&conditions, &updates);

    let mut sum_of_middle = 0;

    for c_up in valid_updates {
        let l = c_up.len();
        assert!(l % 2 == 1);
        sum_of_middle += c_up[l/2];
    }

    let mut sum_of_middle_cor = 0;
    for c_up in corrected_updates.iter() {
        let l = c_up.len();
        assert!(l % 2 == 1);
        sum_of_middle_cor += c_up[l/2];
        
    }

    println!("Valid Updates : {}", sum_of_middle);
    println!("Corrected Updates : {:?}", sum_of_middle_cor);
    Ok(())
}


fn parse_file(path: &str) -> Result<(HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>)> {
    
    let file = BufReader::new(File::open(path)?);
    
    let mut updates = Vec::new();
    let mut conditions = HashMap::new();

    let mut done_with_conditions = false;

    for line in file.lines() {
        let line = line?;

        if line.is_empty() { done_with_conditions = true; continue;}
        if done_with_conditions {
            let update: Vec<_> = line.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
            updates.push(update);            
        }
        else {
            let (prev, next) = line.split_once('|')
                .map(|(f, l)| 
                      (f.parse::<u64>().unwrap(), l.parse::<u64>().unwrap()))
                .unwrap();
            conditions.entry(prev)
                      .and_modify(|nexts: &mut HashSet<u64>| { nexts.insert(next); })
                      .or_insert(HashSet::from([next]));
        }
    }

    Ok((conditions, updates))
}


fn find_valid_updates_and_correct_the_rest<'a>(
    conditions: &HashMap<u64, HashSet<u64>>, updates: &'a Vec<Vec<u64>>
    ) -> (Vec<&'a Vec<u64>>, Vec<Vec<u64>>) {

    let mut valid_updates = Vec::new();
    let mut corrected_updates = Vec::new();
    for update in updates {
        let mut needed_correcting = false;
        'valid_check: for i in 0..update.len() - 1 {
            let prev = update[i];
            for j in i + 1..update.len() {
                let next = update[j];
        
                if conditions.get(&next).map(|s| s.contains(&prev)).is_some_and(|x| x) {
                    needed_correcting = true;
                    break 'valid_check;
                }
            }
        }
        if needed_correcting {
            let mut update_c = update.clone();
            update_c.sort_by(|a, b| {
                let mut c_a = 0;
                let mut c_b = 0; 
                let after_a = conditions.get(&a);
                let after_b = conditions.get(&b);
                for x in update {
                    c_a += after_a.map_or(false, |s| s.contains(x)) as i32;
                    c_b += after_b.map_or(false, |s| s.contains(x)) as i32;
                }
                c_b.cmp(&c_a)
            });
            corrected_updates.push(update_c); 
        }
        else { valid_updates.push(update); }
    }
    return (valid_updates, corrected_updates);
}