use std::{ops::Deref, slice::Iter};

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

    let mut converted: Vec<Option<Num>> = input.chunks(2).enumerate().map(|(chunk_id, chunk)| {
        let file_len = chunk[0];
        let free_space_len = chunk.get(1);
        let ids = vec![Some(chunk_id)].into_iter().cycle().take(file_len);
        let result: Vec<Option<Num>> = match free_space_len {
            Some(l) => {
                let free_spaces = vec![None].into_iter().cycle().take(*l);
                ids.chain(free_spaces).collect()
                //ids.collect()
            },
            _ => ids.collect()
        };
        result
    }).flatten().collect();

    let ordered_items: Vec<Option<Num>> = converted.clone().into_iter().enumerate().map(|(index, item)| {
        println!("{:?}", item);
        match item {
            // TODO:: We want do the fetch from the CONVERTED MUTABLE VECTOR and return THAT value. NOT the ORIGINAL value.
            Some(value) => Some(value),
            None => {
                let mut converted_search = converted.iter();
                match converted_search.rposition(|item| item.is_some()) {
                    Some(rightmost_value_index) => {
                        // Super hacky, but: 
                        // We know rposition index exists because its already evaluated as some,
                        // We know that the item at that position is Some because that's the predicate for rposition
                        // So, flatten and unwrap should never panic :fingers-crossed:
                        println!("{:?}", converted);
                        let rightmost_value: Num = converted.get(rightmost_value_index).unwrap().deref().unwrap();
                        converted.swap(index, rightmost_value_index);
                        Some(rightmost_value)
                    }
                    _ => {
                        None
                    }
                }
            }
        }
    }).collect();
    /*
    converted.clone().iter().for_each(|item| {
        match item {
            Some(num) => print!("{}", num),
            _ => print!(".")
        }
    });
     */
    ordered_items.iter().for_each(|item| {
        match item {
            Some(num) => print!("{}", num),
            _ => print!(".")
        }
    });

    // println!("{:?}", converted);
}
