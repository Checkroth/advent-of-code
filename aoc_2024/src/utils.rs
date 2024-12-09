use core::fmt;
/// Things shared from day to day
/// A lot of this was copied from week1.rs
/// I have opted not to refactor week1 to use these utils, though.
use std::{ cell::Cell, fs, slice::Iter, usize };
use colored::{ColoredString, Colorize};
use itertools::{self, Itertools};

pub fn read_input_as_lines(input_filename: &str) -> Vec<String> {
    let contents = fs
        ::read_to_string(format!("input/{}", input_filename))
        .expect("input not found");
    contents.split("\n").map(String::from).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn from_index(x: usize, y: usize) -> Coord {
        // Helper constructor to handle enumerator usize to i32 constructions
        Coord {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    fn coord_shift(&self) -> Coord {
        match *self {
            Direction::N => Coord { x: 0, y: -1 },
            Direction::NE => Coord { x: 1, y: -1 },
            Direction::E => Coord { x: 1, y: 0 },
            Direction::SE => Coord { x: 1, y: 1 },
            Direction::S => Coord { x: 0, y: 1 },
            Direction::SW => Coord { x: -1, y: 1 },
            Direction::W => Coord { x: -1, y: 0 },
            Direction::NW => Coord { x: -1, y: -1 },
        }
    }

    fn jump_cell(&self, origin: Coord) -> Coord {
        let modify_coord = self.coord_shift();
        Coord {
            x: origin.x + modify_coord.x,
            y: origin.y + modify_coord.y,
        }
    }

    fn rotate_90(&self) -> Option<Direction> {
        // This could probably be done with iterator followed by rotate_left or right, but opting not to do that for now.
        match self {
            Direction::N => Some(Direction::E),
            Direction::E => Some(Direction::S),
            Direction::S => Some(Direction::W),
            Direction::W => Some(Direction::N),
            _ => None,
        }
    }
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ];
        DIRECTIONS.iter()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub symbol: char,
    pub marked: bool,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub height: i32,
    pub width: i32,
    pub cells: Vec<Vec<Node>>,
}

impl Grid {
    pub fn build_from_file(filename: &str) -> Grid {
        let lines = read_input_as_lines(filename);
        let cells: Vec<Vec<Node>> = lines
            .into_iter()
            .map(|row: String| {
                // Each row becomes its own vector of usize, for following calculations
                row.chars()
                    .map(|symbol: char| Node { symbol: symbol, marked: false })
                    .collect()
            })
            .filter(|report: &Vec<Node>| report.len() > 0)
            .collect();

        let grid_height: i32 = (cells.len() as i32) - 1;
        let grid_width: i32 = (cells.first().unwrap().len() as i32) - 1;

        Grid {
            height: grid_height,
            width: grid_width,
            cells: cells,
        }
    }

    pub fn get_cell(&self, position: Coord) -> Option<&Node> {
        let x = self.cells
            .get(position.y as usize)
            .map(|row| row.get(position.x as usize))
            .flatten();

        x
    }

    pub fn mark_cell(&mut self, position: Coord) -> bool {
        match self.get_cell(position.clone()) {
            Some(Node { symbol, marked: false }) => {
                // If get_cell actually gets something, positions must be convertable to usize by definition.
                self.cells[position.y as usize][position.x as usize] = Node {
                    symbol: *symbol,
                    marked: true,
                };
                true
            }
            _ => false,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells
            .iter()
            .for_each(|row| {
                row.iter().for_each(|node| {
                    if node.marked {
                        write!(f, "{}", node.symbol.to_string().on_red());
                    } else {
                        write!(f, "{}", node.symbol.to_string().green());
                    }
                });
                vec![writeln!(f, "")];
                
            });
            Ok(())            
        }
        // Below is a safer, smarter implementation that I couldn't get to work, but leaving for potential future use.
/*     fn __fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells
            .iter()
            .map(|row| {
                let outline = row.iter().map(|node| {
                    if node.marked {
                        write!(f, "{}", node.symbol.to_string().red())
                    } else {
                        write!(f, "{}", node.symbol.to_string().green())
                    }
                });
                outline.chain(vec![writeln!(f, "")]).collect()
                
            }).collect()
        }
        */
}
