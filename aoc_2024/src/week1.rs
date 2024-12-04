use std::{collections::HashMap, fs};


fn insert_str_as_int_in_order(ordered_vec: &mut Vec<i32>, new_item: &str) {
    let parsed = new_item.parse::<i32>().unwrap();
    let position = ordered_vec.binary_search(&parsed).unwrap_or_else(|err| err);
    ordered_vec.insert(position, parsed);
}

/// Reads the input file as a vector of strings.
/// Utility for all aoc problems, as they all begin with a huge text file.
/// All further processing and formatting is highly dependent on the problem, so it ends there.
/// Would technically be more efficient to handle a Vec<&str> instead of converting to String objects,
///     but for re-usability it's a necessary step.
fn read_input_as_lines(input_filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(format!("input/{}", input_filename)).expect("input not found");
    contents.split("\n").map(String::from).collect()
}

pub fn day1() {
    let lines = read_input_as_lines("day1_input.txt");
    // Mutable vectors for ease of building from parsed input
    let mut left_col: Vec<i32> = Vec::new();
    let mut right_col: Vec<i32> = Vec::new();

    for line in lines.iter() {
        line.split_once("   ").map(|(l, r)| {
            insert_str_as_int_in_order(&mut left_col, l);
            insert_str_as_int_in_order(&mut right_col, r);
        });
    }

    let distances = left_col.iter().zip(right_col.iter()).map(|(l, r)| {
        (l - r).abs()
    });

    // Keeps pre-calculated hashmap in case there lots of duplicates in the input that don't need to be re-calculated every time.
    let mut calculated: HashMap<&i32, usize> = HashMap::new();
    let similarity = left_col.iter().map(|item| {
        // get() returns an Optional reference
        // map() dereferences that reference
        // unwrwap_or_else calculates the new usize in the event of a None from get()
        //    This dance is required in order to keep the expression as one type: a dereferences `usize`.
        let multipler: usize = calculated.get(item).map(|item| *item).unwrap_or_else(|| {
            let &count = &right_col.iter().filter(|&n| *n == *item).count();
            count
        });
        calculated.insert(item, multipler);
        item * (multipler as i32)
    });
    let distances_sum: i32 = distances.sum();
    println!("Part 1 (Distances): {}", distances_sum);
    let similarity_sum: i32 = similarity.sum();
    println!("Part 2 (Similarity Score): {}", similarity_sum);

    assert_eq!(distances_sum, 1722302);
    assert_eq!(similarity_sum, 20373490);
}

pub fn day2() {
    println!("Day 2 WIP")
}