use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::i128;
use std::io::{Result, BufRead, BufReader};
use std::iter::repeat_n;

#[allow(unused)]
fn main() -> Result<()> {
    // let (mut disk_map, mut empty_blocks) = parse_input_simple("input.txt")?;

    // println!("{:?}", disk_map);
    // println!("{:?}", empty_blocks);

    // let mut empty_blocks_flat: BTreeSet<usize>  = BTreeSet::from_iter(
        // empty_blocks.iter().flat_map(|(start, len)| *start..(*start + *len))
    // );

    // fill_empty_blocks(&mut disk_map, &mut empty_blocks);

    let (
        mut disk_map,
        file_locations,
        mut empty_blocks
    ) = parse_input_advanced("input.txt")?;

    move_files_to_contiguous_mem(&mut disk_map, file_locations, &mut empty_blocks);

    // println!("{:?}", disk_map);
    // println!("{:?}", empty_blocks);


    //println!("{:?}", disk_map);
    let disk_checksum : _ = disk_map.iter().enumerate()
        .filter_map(|(index, block_id)| {
            (*block_id != -1).then_some(index as i128 * *block_id)
        }).sum::<i128>();

    println!("Checksum: {}", disk_checksum);
    return Ok(());
}

fn move_files_to_contiguous_mem(
    disk_map: &mut Vec<i128>,
    file_locations: Vec<(i128, (usize, usize))>,
    empty_blocks: &mut BTreeMap<usize, usize>,
) {
    for (file_id, (file_start, file_len)) in file_locations.into_iter().rev() {

        // println!("{empty_blocks:?}");
        let big_enough_space = empty_blocks.iter()
            .filter(|(position, capacity)| {
                **position <= file_start && **capacity >= file_len
            }).next();

        // println!("{file_id}, {file_start}, {file_len}");
        // println!("{big_enough_space:?}");
        // println!("{disk_map:?}");


        if let Some(non_empty_space) = big_enough_space {
            let capacity = *non_empty_space.1;
            let first_pos = *non_empty_space.0;
            if first_pos >= file_start { println!("Shouldn't happen now"); continue; }

            // Now move the file into this empty space.
            // Check that the space is empty
            assert!(disk_map[first_pos..first_pos + capacity].iter().all(|&x| x == -1));
            // Fill the space
            disk_map[first_pos..first_pos + file_len].fill(file_id);
            // Empty the old space
            disk_map[file_start..file_start + file_len].fill(-1);
            // Remove the filled space
            empty_blocks.remove(&first_pos);

            // Add a new empty blcok
            if capacity - file_len <= 0 {continue;}
            let new_capacity = capacity - file_len;
            empty_blocks.insert(first_pos + file_len, new_capacity);

        }

    }
}


fn fill_empty_blocks(disk_map: &mut Vec<i64>, empty_blocks: &mut BTreeSet<usize>) {
    let disk_size = disk_map.len();
    loop {
        let empty_index = empty_blocks.pop_first().unwrap();
        assert!(disk_map[empty_index] == -1);
        let (index, &last_filled_block) = disk_map.iter().rev().enumerate()
            .filter(|(_, x)| {**x != -1}).next().unwrap();

        if disk_size - index - 1 < empty_index { break; }
        // println!("Empty block {}, Last filled block value {}, last filled block index {}",
        //    empty_index, last_filled_block, disk_size - 1 - index);

        disk_map[empty_index] = last_filled_block;
        disk_map[disk_size - index - 1] = -1;
        empty_blocks.insert(disk_size - index - 1);
    }
}

fn parse_input_advanced(path: &str) -> Result<(Vec<i128>, Vec<(i128, (usize, usize))>,  BTreeMap<usize, usize>)> {
    let mut file = BufReader::new(File::open(path)?);
    let mut disk_map = vec![];
    let mut empty_blocks = BTreeMap::new();
    let mut file_locations = Vec::new();
    let mut counter = 0;
    let mut file_contents = String::new();
    let _ = file.read_line(&mut file_contents)?;
    for c in file_contents.chars() {
        let c = c.to_digit(10).unwrap() as usize;
        if counter % 2 == 1 {
            if c !=0 {
                empty_blocks.insert(disk_map.len(), c);
            }
            disk_map.extend(repeat_n(-1, c));
        }
        else {
            file_locations.push((counter / 2, (disk_map.len(), c)));
            disk_map.extend(repeat_n(counter / 2, c));
        }
        counter += 1;
    }
    return Ok((disk_map, file_locations, empty_blocks));
}

fn parse_input_simple(path: &str) -> Result<(Vec<i64>, BTreeSet<usize>)> {
    let mut file = BufReader::new(File::open(path)?);
    let mut disk_map = vec![];
    let mut empty_blocks = BTreeSet::new();

    let mut counter = 0;
    let mut file_contents = String::new();
    let _ = file.read_line(&mut file_contents)?;
    for c in file_contents.chars() {
        let c = c.to_digit(10).unwrap() as usize;
        if counter % 2 == 1 {
            empty_blocks.extend(disk_map.len()..(disk_map.len() + c));
            disk_map.extend(repeat_n(-1, c));
        }
        else {
            disk_map.extend(repeat_n(counter / 2, c));
        }
        counter += 1;
    }
    return Ok((disk_map, empty_blocks));
}