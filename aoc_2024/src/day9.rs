use core::fmt;
use std::{ collections::{ HashMap, HashSet }, ops::Deref, path::Display, slice::Iter };

use itertools::Itertools;

use crate::utils;

type Num = usize;

pub fn part1() {
    // This problem only has 1 line of input
    let input: Vec<Num> = utils
        ::read_input_as_lines("day9_input.txt")
        .first()
        .unwrap()
        .chars()
        .map(|item| item.to_string().parse::<Num>().unwrap())
        .collect();

    let mut converted: Vec<Option<Num>> = input
        .chunks(2)
        .enumerate()
        .map(|(chunk_id, chunk)| {
            let file_len = chunk[0];
            let free_space_len = chunk.get(1);
            let ids = vec![Some(chunk_id)].into_iter().cycle().take(file_len);
            let result: Vec<Option<Num>> = match free_space_len {
                Some(l) => {
                    let free_spaces = vec![None].into_iter().cycle().take(*l);
                    ids.chain(free_spaces).collect()
                    //ids.collect()
                }
                _ => ids.collect(),
            };
            result
        })
        .flatten()
        .collect();
    let original_converted = converted.clone();

    let ordered_items: Vec<Option<Num>> = converted
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            match item {
                Some(value) => *converted.get(index).unwrap(),
                None => {
                    let mut converted_search = converted.iter();
                    match converted_search.rposition(|item| item.is_some()) {
                        Some(rightmost_value_index) => {
                            // Super hacky, but:
                            // We know rposition index exists because its already evaluated as some,
                            // We know that the item at that position is Some because that's the predicate for rposition
                            // So, flatten and unwrap should never panic :fingers-crossed:
                            if rightmost_value_index > index {
                                let rightmost_value: Num = converted
                                    .get(rightmost_value_index)
                                    .unwrap()
                                    .deref()
                                    .unwrap();
                                converted.swap(index, rightmost_value_index);
                                Some(rightmost_value)
                            } else {
                                None
                            }
                        }
                        _ => { None }
                    }
                }
            }
        })
        .collect();
    /*
    converted.clone().iter().for_each(|item| {
        match item {
            Some(num) => print!("{}", num),
            _ => print!(".")
        }
    });
     */
    original_converted.iter().for_each(|item| {
        match item {
            Some(num) => print!("{}", num),
            _ => print!("."),
        }
    });
    println!("");
    ordered_items.iter().for_each(|item| {
        match item {
            Some(num) => print!("{}", num),
            _ => print!("."),
        }
    });
    println!("");
    let total: usize = ordered_items
        .iter()
        .flatten()
        .enumerate()
        .map(|(index, value)| { index * value })
        .sum();
    println!("total sum: {}", total);
}

pub fn part2() {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Fragment {
        file_id: usize,
        file_multiplier: usize,
        free_space: usize,
        consumed_free_space: usize,
        moved: bool,
        appended_fragments: Vec<Fragment>,
    }

    impl fmt::Display for Fragment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let x: String = (0..self.file_multiplier).map(|_| self.file_id.to_string()).join("");
            write!(f, "{}", x)
        }
    }
    // This problem only has 1 line of input
    let input: Vec<Num> = utils
        ::read_input_as_lines("day9_input.txt")
        .first()
        .unwrap()
        .chars()
        .map(|item| item.to_string().parse::<Num>().unwrap())
        .collect();

    let mut converted: Vec<Fragment> = input
        .chunks(2)
        .enumerate()
        .map(|(chunk_id, chunk)| {
            let file_len = chunk[0];
            println!("chunk: {:?}", chunk);
            let free_space_len = match chunk.get(1) {
                Some(l) => *l,
                _ => 0,
            };
            Fragment {
                file_id: chunk_id,
                file_multiplier: file_len,
                free_space: free_space_len,
                consumed_free_space: 0,
                moved: false,
                appended_fragments: Vec::new(),
            }
        })
        .collect();

    let mut main_reference = converted.clone();
    let mut insertions: HashMap<Num, Vec<Num>> = HashMap::new();
    converted
        .iter_mut()
        .rev()
        .enumerate()
        .map(|(index, frag)| {
            let open_spot = main_reference
                .iter()
                .position(|item| frag.file_multiplier <= item.consumed_free_space);
            match open_spot {
                Some(spot) => {
                    let item = main_reference.get_mut(spot).unwrap();
                    item.consumed_free_space += frag.file_multiplier;
                    item.appended_fragments.push(frag.clone());

                    let moved_frag = main_reference.get_mut(index).unwrap();
                    moved_frag.moved = true;
                }
                None => (),
            }
        })
        .collect::<Vec<_>>();
    let result: Vec<Num> = main_reference.into_iter().flat_map(|frag| {
        let followers = frag.appended_fragments
            .into_iter()
            .flat_map(|appended_frag| {
                (0..appended_frag.file_multiplier).map(|_| appended_frag.file_id)
            });
        let leaders = (0..frag.file_multiplier).map(|_| frag.file_id);
        leaders.chain(followers).collect::<Vec<usize>>()
    }).collect();

    
    let total: usize = result
        .into_iter()
        .enumerate()
        .map(|(index, value)| { index * value })
        .sum();
    println!("total sum: {}", total);

    /* 
    fn find_farthest_matches(
        free_space: usize,
        current_index: usize,
        remainder: &mut Vec<Fragment>
    ) -> Vec<Fragment> {
        let farthest_index = remainder.iter().rposition(|item| !item.moved && (item.file_multiplier <= free_space));
        match farthest_index {
            Some(index) => {
                if index < current_index {
                    Vec::new()
                } else {
                    let item = remainder.remove(index);
                    println!("removed item: {:?}", item);
                    //remainder.iter().for_each(|item| { print!("{}", item)});
                    //println!("");
                    let remaining_amount = free_space - item.file_multiplier;
                    vec![item]
                        .into_iter()
                        .chain(find_farthest_matches(remaining_amount, current_index, remainder))
                        .collect()
                }
            }
            None => { Vec::new() }
        }
    }
    
    let mut main_reference = converted.clone();
    let mut moved_nodes: HashSet<Num> = HashSet::new();
    let reordered: Vec<Num> = converted
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let preprocessed = moved_nodes.get(&item.file_id);
            match preprocessed {
                Some(_) => Vec::new(),
                None => {
                    let matches = find_farthest_matches(item.free_space, index, &mut main_reference);
                    let insertion: Vec<Fragment> = vec![item.clone()].into_iter().chain(matches.into_iter()).collect();
                    insertion.iter().for_each(|item| {
                        moved_nodes.insert(item.file_id);
                    });
                    // let ids = vec![Some(chunk_id)].into_iter().cycle().take(file_len);
                    insertion.into_iter().map(|item| {
                        vec![item.file_id].into_iter().cycle().take(item.file_multiplier)
                    }).flatten().collect()
                }
            }
        }).flatten().collect();

    reordered.iter().for_each(|item| {
        print!("{}", item);
        });

    let total: usize = reordered
        .into_iter()
        .enumerate()
        .map(|(index, value)| { index * value })
        .sum();
    println!("total sum: {}", total);
    */
}
