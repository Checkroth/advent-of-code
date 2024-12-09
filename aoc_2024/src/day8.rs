use itertools::Positions;

use crate::utils::{self, Coord};
use std::{ collections::HashMap, collections::HashSet };

pub fn part1() {
    let mut antennas: HashMap<char, HashSet<utils::Coord>> = HashMap::new();
    let mut grid = utils::Grid::build_from_file("day8_input.txt");
    // Build hashmap of antenna locations
    // This could be done inside build_from_file to improve performance, but left separate for reusability
    grid.cells.iter().enumerate().for_each(|(y_index, row)| {
        row.iter().enumerate().for_each(|(x_index, node)| {
            let position = utils::Coord::from_index(x_index, y_index);
            match node {
                utils::Node { symbol: '.', marked: _} => (),
                node => {
                    match antennas.get_mut(&node.symbol) {
                        Some(frequencies) => {
                            frequencies.insert(position);
                        },
                        _ => {
                            antennas.insert(node.symbol, HashSet::from([position]));
                        }
                    }
                }
            }
        });
    });

    antennas.into_iter().for_each(|(symbol, locations)| {
        locations.iter().for_each(|source_position| {
            locations.iter().for_each(|coord| {
                let x_diff = source_position.x - coord.x;
                let y_diff = source_position.y - coord.y;
                if (x_diff.abs() + y_diff.abs()) > 1 {
                    let first_antinode = Coord {
                        x: source_position.x + -(x_diff * 2),
                        y: source_position.y + -(y_diff * 2)
                    };
                    let second_antinode = Coord {
                        x: coord.x + (x_diff * 2),
                        y: coord.y + (y_diff * 2)
                    };
                    grid.mark_cell(first_antinode);
                    grid.mark_cell(second_antinode);
                }
            });
        })
    });

    let marked_cells = grid.cells.iter().flat_map(|row| {
        row.iter().filter(|node| node.marked)
    });

    println!("{}", grid);
    println!("Part 1: Num marked cells: {}", marked_cells.count());

}

pub fn part2() {
    let mut antennas: HashMap<char, HashSet<utils::Coord>> = HashMap::new();
    let mut grid = utils::Grid::build_from_file("day8_input.txt");
    // Build hashmap of antenna locations
    // This could be done inside build_from_file to improve performance, but left separate for reusability
    grid.cells.iter().enumerate().for_each(|(y_index, row)| {
        row.iter().enumerate().for_each(|(x_index, node)| {
            let position = utils::Coord::from_index(x_index, y_index);
            match node {
                utils::Node { symbol: '.', marked: _} => (),
                node => {
                    match antennas.get_mut(&node.symbol) {
                        Some(frequencies) => {
                            frequencies.insert(position);
                        },
                        _ => {
                            antennas.insert(node.symbol, HashSet::from([position]));
                        }
                    }
                }
            }
        });
    });

    fn mark_chain(grid: &mut utils::Grid, origin: utils::Coord, offset: utils::Coord) {
        grid.mark_cell(origin.clone());
        let next_cell = utils::Coord {
            x: origin.x + offset.x,
            y: origin.y + offset.y
        };
        if grid.get_cell(next_cell.clone()).is_some() {
            mark_chain(grid, next_cell, offset)
        }
    }
    antennas.into_iter().for_each(|(symbol, locations)| {
        locations.iter().for_each(|source_position| {
            locations.iter().for_each(|coord| {
                let x_diff = source_position.x - coord.x;
                let y_diff = source_position.y - coord.y;
                
                if (x_diff.abs() + y_diff.abs()) > 0 {
                    let first_offset = Coord {
                        x: -x_diff,
                        y: -y_diff
                    };
                    mark_chain(&mut grid, coord.clone(), first_offset);
                    let second_offset = Coord {
                        x: x_diff,
                        y: y_diff
                    };
                    mark_chain(&mut grid, source_position.clone(), second_offset);
                }
            });
        })
    });

    let marked_cells = grid.cells.iter().flat_map(|row| {
        row.iter().filter(|node| node.marked)
    });

    println!("{}", grid);
    println!("Part 2: Num marked cells repeating: {}", marked_cells.count());

}