use std::{ collections::HashMap, f32::consts::E, fs, hash::Hash, iter, result, slice::Iter, usize };
use regex::{ Regex, Match };
use std::cmp::Ordering;

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
    let contents = fs
        ::read_to_string(format!("input/{}", input_filename))
        .expect("input not found");
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

    let distances = left_col
        .iter()
        .zip(right_col.iter())
        .map(|(l, r)| (l - r).abs());

    // Keeps pre-calculated hashmap in case there lots of duplicates in the input that don't need to be re-calculated every time.
    let mut calculated: HashMap<&i32, usize> = HashMap::new();
    let similarity = left_col.iter().map(|item| {
        // get() returns an Optional reference
        // map() dereferences that reference
        // unwrwap_or_else calculates the new usize in the event of a None from get()
        //    This dance is required in order to keep the expression as one type: a dereferences `usize`.
        let multipler: usize = calculated
            .get(item)
            .map(|item| *item)
            .unwrap_or_else(|| {
                let &count = &right_col
                    .iter()
                    .filter(|&n| *n == *item)
                    .count();
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

/// Day2 Rules
/// Each row is marked "Safe" or "Not Safe"
/// Answer is the number of "Safe" rows.
/// Criteria to be safe:
///     1. The entire row is either deacreasing or increasing (1 > 2 > 3 or 3 > 2 > 1, never 2 > 3 > 1)
///     2. Each column of the row differs by at least 1 from the previous
///     3. Each column of the row differs by at most 3 from the previous
pub fn day2() {
    let lines = read_input_as_lines("day2_input.txt");
    // TODO:: These collects are wasteful -- we we should be able to hand the raw streams around...
    let reports: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|row: String| {
            // Each row becomes its own vector of usize, for following calculations
            row.split_whitespace()
                .flat_map(|room: &str| room.parse::<i32>())
                .collect()
        })
        .filter(|report: &Vec<i32>| report.len() > 0)
        .collect();

    struct ReportSafety {
        is_safe: bool,
        previous_room: Option<i32>,
        is_ascending: Option<bool>,
    }

    struct EvaluatedReport {
        report: Vec<i32>,
        report_safety: ReportSafety,
    }

    fn is_report_safe(report: Vec<i32>) -> ReportSafety {
        report.iter().fold(
            ReportSafety {
                is_safe: true,
                previous_room: None,
                is_ascending: None,
            },
            |safety_report, room| {
                let new_safety_report = safety_report.previous_room.map(|previous_room| {
                    let is_ascending = previous_room < *room;
                    let room_diff = (previous_room - room).abs();
                    let room_diff_safe = 1 <= room_diff && room_diff <= 3;
                    let ascention_safe = safety_report.is_ascending
                        .map(|b| b == is_ascending)
                        .unwrap_or(true);
                    // The new report compounds on the previous report; Safety never switch false to true, but could switch False here.
                    // The previous room is set to the current processing room, for the next iteration of this fold.
                    let is_safe = room_diff_safe && ascention_safe && safety_report.is_safe;
                    ReportSafety {
                        is_safe: is_safe,
                        previous_room: Some(*room),
                        is_ascending: Some(is_ascending),
                    }
                });

                new_safety_report.unwrap_or(ReportSafety {
                    is_safe: safety_report.is_safe,
                    previous_room: Some(*room),
                    is_ascending: safety_report.is_ascending,
                })
            }
        )
    }

    let num_total_reports = reports.len();
    let report_safety_metrics: Vec<EvaluatedReport> = reports
        .into_iter()
        .map(|report| {
            EvaluatedReport {
                report: report.clone(),
                report_safety: is_report_safe(report),
            }
        })
        .collect();

    // Safe when dampened was added after the fact, which is why the EvaluatedReport response structure doesn't really make sense here.
    // The original goal was to evaluate safe and safe-when-dampened in the same evaluation by maintaining whether or not we had skipped already during the fold.
    // This didn't work because there are patterns where skipping the unsafe room doesn't make the report safe, but skipping another would.
    // For instance, 2 1 2 3 4 5: Skipping in-place would skip 1, keeping this report unsafe. Skipping the first 2, however, makes it safe.
    // The fallback solution here is a dirty hack that just attempts to drop each item from the report one by one until one is potentially marked "safe".
    // There is almost certainly a smarter solution, but such is life
    let safe_when_dampened = report_safety_metrics
        .iter()
        .filter(|report| !report.report_safety.is_safe)
        .map(|unsafe_report| {
            for i in 0..unsafe_report.report.len() {
                let mut partial_report = unsafe_report.report.clone();
                partial_report.remove(i);
                let partial_evaluation = is_report_safe(partial_report.clone());
                if partial_evaluation.is_safe {
                    return EvaluatedReport {
                        report: partial_report,
                        report_safety: partial_evaluation,
                    };
                }
            }
            EvaluatedReport {
                report: unsafe_report.report.clone(),
                report_safety: ReportSafety {
                    is_safe: false,
                    is_ascending: None,
                    previous_room: None,
                },
            }
        });

    let num_safe_reports = report_safety_metrics
        .iter()
        .filter(|report| report.report_safety.is_safe)
        .count();
    let num_dampened_safe_reports = safe_when_dampened
        .into_iter()
        .filter(|report| report.report_safety.is_safe)
        .count();

    println!("Num Reports): {}", num_total_reports);
    println!("Part 1 (Num Safe Reports): {}", num_safe_reports);
    assert_eq!(num_safe_reports, 526);
    println!(
        "Part 2 (Num Dampened Safe Reports): {}",
        num_dampened_safe_reports + num_safe_reports
    );
    // Don't have the answer yet, but know its more than this
    assert!(num_dampened_safe_reports + num_safe_reports > 552);

    ()
}

pub fn day3() {
    let lines = read_input_as_lines("day3_input.txt");
    let input = lines.join("");
    let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    // let inner_regex = Regex::new(r"\d{1,3}").unwrap();
    fn multiply_match(m: Match<'_>) -> i32 {
        let raw: &str = m.as_str();
        // TODO:: this is a pain in the ass to turn in to a constant available outside fn ref so oh well, eat it performance
        let inner_regex = Regex::new(r"\d{1,3}").unwrap();
        inner_regex
            .find_iter(raw)
            .fold(1, |result, m| { result * m.as_str().parse::<i32>().unwrap() })
    }

    let multiplied_values = mul_regex.find_iter(&input).map(multiply_match);

    struct OperationalHoldover {
        doing: bool,
        total: i32,
    }

    // TODO / another option: Named capture matches? But, maybe more complicated than its worth...
    let doanddont_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let multipled_values_with_holdover = doanddont_regex
        .find_iter(&input)
        .fold(OperationalHoldover { doing: true, total: 0 }, |holdover, m| {
            let raw: &str = m.as_str();
            match raw {
                "do()" => OperationalHoldover { doing: true, total: holdover.total },
                "don't()" => OperationalHoldover { doing: false, total: holdover.total },
                _ =>
                    match holdover.doing {
                        true =>
                            OperationalHoldover {
                                doing: true,
                                total: holdover.total + multiply_match(m),
                            },
                        false => holdover,
                    }
            }
        });

    let part_1_sum = multiplied_values.sum::<i32>();
    println!("Part 1 (sum of mults): {}", part_1_sum);
    assert_eq!(part_1_sum, 166630675);
    println!("Part 2 (do's and don'ts): {}", multipled_values_with_holdover.total)
}

const XMAS: &str = "XMAS";
pub fn _day4_snake() {
    let lines = read_input_as_lines("day4_input.txt");
    #[derive(Clone, Copy, Debug)]
    struct Node {
        letter: char,
        visited: bool,
    }

    #[derive(Clone, Debug)]
    struct Grid {
        max_x: i32,
        max_y: i32,
        cells: Vec<Vec<Node>>,
    }
    let cells: Vec<Vec<Node>> = lines
        .into_iter()
        .map(|row: String| {
            // Each row becomes its own vector of usize, for following calculations
            row.chars()
                .map(|letter: char| Node { letter: letter, visited: false })
                .collect()
        })
        .filter(|report: &Vec<Node>| report.len() > 0)
        .collect();

    let grid_height: i32 = (cells.len() as i32) - 1;
    let grid_width: i32 = (cells.first().unwrap().len() as i32) - 1;

    let grid = Grid {
        max_x: grid_height,
        max_y: grid_width,
        cells: cells,
    };

    // The Crawl: Append <the crawl> to the end of the string  (4 chars), return a list of crawls
    fn build_fourpairs(grid: Grid, current_node: (i32, i32), current_word: String) -> Vec<String> {
        println!("{}", current_word);
        let mut adjacent_nodes: Vec<(usize, usize)> = Vec::new();
        let (x, y) = current_node;
        let current_cell = grid.cells
            .get(y as usize)
            .map(|row| row.get(x as usize))
            .flatten();
        current_cell
            .map(|node| {
                match node.visited {
                    true => None,
                    false => {
                        let mut inner_grid = grid.clone();
                        inner_grid.cells[y as usize][x as usize] = Node {
                            letter: node.letter,
                            visited: true,
                        };
                        let mut new_word = current_word.to_owned();
                        new_word.push(node.letter);
                        if XMAS.starts_with(&new_word) {
                            let mut results: Vec<String> = Vec::new();
                            match new_word.len() {
                                4 => results.append(Vec::from([new_word]).as_mut()),
                                _ => {
                                    for y_offset in -1..=1 {
                                        let new_y = y + y_offset;
                                        if 0 <= new_y && new_y <= grid.max_y {
                                            for x_offset in -1..=1 {
                                                let new_x = x + x_offset;
                                                results.append(
                                                    build_fourpairs(
                                                        inner_grid.clone(),
                                                        (new_x, new_y),
                                                        new_word.clone()
                                                    ).as_mut()
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            Some(results)
                        } else {
                            None
                        }
                    }
                }
            })
            .flatten()
            .unwrap_or(Vec::new())
    }

    let mut all_words: Vec<String> = Vec::new();
    for y in 0..=grid_height {
        for x in 0..=grid_width {
            let grid_iteration = grid.clone();
            all_words.append(build_fourpairs(grid_iteration, (x, y), String::new()).as_mut());
        }
    }

    let count_xmas = all_words
        .into_iter()
        .filter(|word| *word == String::from("XMAS"))
        .count();
    println!("Part 1 (count all xmas): {}", count_xmas)
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
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
    fn coord_shift(&self) -> (i32, i32) {
        match *self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
    fn jump_cell(&self, origin: (i32, i32)) -> (i32, i32) {
        let (origin_x, origin_y) = origin;
        let (modify_x, modify_y) = self.coord_shift();
        (origin_x + modify_x, origin_y + modify_y)
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
struct Node {
    letter: char,
    visited: bool,
}

#[derive(Clone, Debug)]
struct Grid {
    height: i32,
    width: i32,
    cells: Vec<Vec<Node>>,
}

impl Grid {
    pub fn build_from_file(filename: &str) -> Grid {
        let lines = read_input_as_lines(filename);
        let cells: Vec<Vec<Node>> = lines
            .into_iter()
            .map(|row: String| {
                // Each row becomes its own vector of usize, for following calculations
                row.chars()
                    .map(|letter: char| Node { letter: letter, visited: false })
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

    fn get_cell(&self, position: (i32, i32)) -> Option<&Node> {
        let (x, y) = position;
        let x = self.cells
            .get(y as usize)
            .map(|row| row.get(x as usize))
            .flatten();

        x
    }
}
pub fn day4() {
    let grid = Grid::build_from_file("day4_input.txt");

    // The Crawl: Append <the crawl> to the end of the string  (4 chars), return a list of crawls
    fn build_fourpairs(
        grid: Grid,
        current_node: (i32, i32),
        current_word: String,
        direction: &Direction
    ) -> Vec<String> {
        let (x, y) = current_node;
        let current_cell = grid.get_cell(current_node);
        current_cell
            .map(|node| {
                match node.visited {
                    true => None,
                    false => {
                        let mut inner_grid = grid.clone();
                        inner_grid.cells[y as usize][x as usize] = Node {
                            letter: node.letter,
                            visited: true,
                        };
                        let mut new_word = current_word.to_owned();
                        new_word.push(node.letter);
                        if XMAS.starts_with(&new_word) {
                            let mut results: Vec<String> = Vec::new();
                            match new_word.len() {
                                4 => results.append(Vec::from([new_word]).as_mut()),
                                _ => {
                                    let (x_offset, y_offset) = direction.coord_shift();
                                    let new_y = y + y_offset;
                                    let new_x = x + x_offset;
                                    results.append(
                                        build_fourpairs(
                                            inner_grid.clone(),
                                            (new_x, new_y),
                                            new_word.clone(),
                                            direction
                                        ).as_mut()
                                    );
                                }
                            }
                            Some(results)
                        } else {
                            None
                        }
                    }
                }
            })
            .flatten()
            .unwrap_or(Vec::new())
    }

    let mut all_words: Vec<String> = Vec::new();
    for y in 0..=grid.height {
        for x in 0..=grid.width {
            Direction::iterator().for_each(|direction| {
                let grid_iteration = grid.clone();
                all_words.append(
                    build_fourpairs(grid_iteration, (x, y), String::new(), direction).as_mut()
                );
            });
        }
    }

    let count_xmas = all_words
        .into_iter()
        .filter(|word| *word == String::from("XMAS"))
        .count();
    println!("Part 1 (count all xmas): {}", count_xmas);
    assert_eq!(count_xmas, 2562);
}

pub fn day4_part2() {
    let grid = Grid::build_from_file("day4_input.txt");

    fn build_crosses(grid: &Grid, origin: (i32, i32)) -> usize {
        let (origin_x, origin_y) = origin;
        let top_left_position = Direction::NW.jump_cell(origin);
        let top_right_position = Direction::NE.jump_cell(origin);
        let bottom_left_position = Direction::SW.jump_cell(origin);
        let bottom_right_position = Direction::SE.jump_cell(origin);

        let maybe_top_left = grid.get_cell(top_left_position);
        let maybe_top_right = grid.get_cell(top_right_position);
        let maybe_bottom_left = grid.get_cell(bottom_left_position);
        let maybe_bottom_right = grid.get_cell(bottom_right_position);
        let maybe_origin = grid.get_cell(origin);

        if
            let (
                Some(top_left),
                Some(top_right),
                Some(bottom_left),
                Some(bottom_right),
                Some(origin_node),
            ) = (
                maybe_top_left,
                maybe_top_right,
                maybe_bottom_left,
                maybe_bottom_right,
                maybe_origin,
            )
        {
            let forward_diag: String = vec![
                top_left.letter,
                origin_node.letter,
                bottom_right.letter
            ]
                .into_iter()
                .collect();
            let back_diag: String = vec![bottom_left.letter, origin_node.letter, top_right.letter]
                .into_iter()
                .collect();
            let forward_hit =
                forward_diag == String::from("MAS") || forward_diag == String::from("SAM");
            let back_hit = back_diag == String::from("MAS") || back_diag == String::from("SAM");

            match (back_hit, forward_hit) {
                (true, true) => 1,
                _ => 0,
            }
        } else {
            0
        }
    }

    let mut count: usize = 0;
    for y in 0..=grid.height {
        for x in 0..=grid.width {
            let position = (x, y);
            let maybe_cell = grid.get_cell(position);
            if let Some(Node { letter: 'A', visited: _ }) = maybe_cell {
                // Only look for crosses if the origin node is an M
                count += build_crosses(&grid, position);
            }
        }
    }
    println!("Day 4 Part 2 (X-MAS): {}", count);
    assert_eq!(count, 1902)
}

pub fn day5() {
    #[derive(Clone, Copy, Debug)]
    struct PageRule {
        first: usize,
        second: usize,
    }

    let mut rules: HashMap<usize, Vec<PageRule>> = HashMap::new();

    let lines = read_input_as_lines("day5_input.txt");
    // Line that input switches over
    let input_pivot = 1177;
    // let input_pivot = 22;
    let (rules_input, printer_input) = lines.split_at(input_pivot);

    rules_input
        .into_iter()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (first_raw, second_raw) = line.split_once("|").unwrap();
            let first = first_raw.parse::<usize>().unwrap();
            let second = second_raw.parse::<usize>().unwrap();
            match rules.get_mut(&first) {
                Some(ruleset) => ruleset.push(PageRule { first, second }),
                _ => {
                    rules.insert(first, vec![PageRule { first, second }]);
                }
            };
        });

    fn row_is_correct(rules: &HashMap<usize, Vec<PageRule>>, pages: &Vec<usize>) -> bool {
        // curried just so we can get the hashmap without having to pass it around :shrug:
        pages
            .iter()
            .enumerate()
            .fold(true, |is_correct, (index, page)| {
                let page_rules = rules.get(&page);
                page_rules
                    .map(|rules| {
                        rules
                            .into_iter()
                            .map(|rule| {
                                // Todo:: Find index of all 'second' rule nums; ensure they are > than max of left index
                                // fold to true if all ok; abort quick on f1alse.
                                let must_follow_indexes = pages
                                    .iter()
                                    .rposition(|item| item == &rule.second)
                                    .map(|found_index| { found_index > index });
                                match must_follow_indexes {
                                    Some(true) => true,
                                    Some(false) => false,
                                    None => true,
                                }
                            })
                            .all(|x| x)
                    })
                    .unwrap_or(true) && is_correct
            })
    }

    let converted_rows: Vec<Vec<usize>> = printer_input
        .into_iter()
        .map(|line| {
            let pages: Vec<usize> = line
                .split(",")
                .map(|raw| raw.parse::<usize>().unwrap())
                .collect();
            pages
        })
        .collect();

    // This could be done in one swoop with the unstable feature drain_filter
    let correct_rows: Vec<Vec<usize>> = converted_rows
        .clone()
        .into_iter()
        .filter(|row| row_is_correct(&rules, row))
        .collect();
    let incorrect_rows: Vec<Vec<usize>> = converted_rows
        .clone()
        .into_iter()
        .filter(|row| !row_is_correct(&rules, row))
        .collect();
    let fixed_rows = incorrect_rows.iter().map(|row| {
        let mut fixed = row.clone();
        fixed.sort_by(|left, right| {
            let left_page_rules = rules.get(&left);
            let right_page_rules = rules.get(&right);
            if let Some(rules) = left_page_rules {
                let relevant_left_rule = rules
                    .iter()
                    .filter(|rule| rule.second == *right)
                    .next();
                if relevant_left_rule.is_some() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else if let Some(rules) = right_page_rules {
                let relevant_right_rule = rules
                    .iter()
                    .filter(|rule| rule.second == *left)
                    .next();
                if relevant_right_rule.is_some() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Less
            }
        });
        fixed
    });

    let correct_mids = correct_rows
        .into_iter()
        .filter_map(|row| { row.get(row.len() / 2).cloned() });

    let fixed_incorrect_mids = fixed_rows
        .into_iter()
        .filter_map(|row| { row.get(row.len() / 2).cloned() });

    // Get the incorrect rows, then fix and count them.
    // It would be cleaner & more efficient to fix them in the first iteration (as well as collect correct & incorrect separately)
    // which I'm only not doing because it's annoying and I don't care about performance

    let just_mids: Vec<usize> = correct_mids.clone().collect();
    println!("correct mids: {:?}", just_mids);
    let sum_mids: usize = correct_mids.sum();
    // let sum_incorrect_mids: usize = other_fixed.sum();
    let sum_incorrect_mids: usize = fixed_incorrect_mids.sum();
    println!("Part 1 (Sum of Correct Mids): {}", sum_mids);
    println!("Part 2 (Sum of Fixed Incorrect Mids): {}", sum_incorrect_mids);
}

pub fn day6() {
    let mut map = Grid::build_from_file("day6_input.txt");
    let start_position = map.cells
        .iter()
        .enumerate()
        .fold(None, |guard_position, (guard_y_position, row)| {
            let guard_x_position = row.iter().position(|x| x.letter == '^');
            match (guard_position, guard_x_position) {
                (None, Some(x)) => Some((x as i32, guard_y_position as i32)),
                (Some(pos), _) => Some(pos),
                _ => None,
            }
        })
        .unwrap();
    println!("Start position: {:?}", start_position);
    println!("Starting Cell: {:?}", map.get_cell(start_position));
    let (start_x, start_y) = start_position;
    // Consider the starting node as already visited
    map.cells[start_y as usize][start_x as usize] = Node { letter: '^', visited: true };
    let p2_map = map.clone();

    fn traverse_map(map: &mut Grid, direction: Direction, position: (i32, i32)) {
        let next_position = direction.jump_cell(position);
        let (next_node_x, next_node_y) = next_position;
        let next_node = map.get_cell(next_position);
        // println!("Going from {:?} to {:?} with direction {:?}", position, next_node, direction);
        match next_node {
            Some(Node { letter: '#', visited: _ }) => {
                traverse_map(map, direction.rotate_90().unwrap(), position);
            }
            Some(node) => {
                map.cells[next_node_y as usize][next_node_x as usize] = Node {
                    letter: 'X',
                    visited: true,
                };
                traverse_map(map, direction, next_position);
            }
            None => {
                // End of the recursion -- we've left the map!
                ()
            }
        }
    }
    traverse_map(&mut map, Direction::N, start_position);

    fn traverse_will_loop(
        map: &Grid,
        direction: Direction,
        position: (i32, i32),
        previous_turns: &mut Vec<((i32, i32), Direction)>
    ) -> bool {
        let next_position = direction.jump_cell(position);
        let next_node = map.get_cell(next_position);
        let rotated = direction.rotate_90().unwrap();
        match next_node {
            Some(Node { letter: '#', visited: _ }) => {
                let already_encountered = previous_turns
                    .iter()
                    .filter(|(prev_position, prev_direction)| {
                        // println!("{:?} == {:?} && {:?} == {:?}", prev_position, position, prev_direction, direction);
                        *prev_position == position && *prev_direction == direction
                    })
                    .next();
                match already_encountered {
                    Some(_) => true,
                    _ => {
                        previous_turns.push((position, direction.clone()));
                        traverse_will_loop(
                            map,
                            direction.rotate_90().unwrap(),
                            position,
                            previous_turns
                        )
                    }
                }
            }
            Some(_) => { traverse_will_loop(map, direction, next_position, previous_turns) }
            None => {
                // End of the recursion -- we've left the map!
                // println!("Traversed paths: {:?}", traversed_paths);
                false
            }
        }
    }

    fn traverse_map_for_loops(
        map: &Grid,
        direction: Direction,
        position: (i32, i32)
    ) -> Vec<(i32, i32)> {
        // TODO:: This currently comes up with 6004 new blockers, which is considered "too high". There's a bug, somewhere.
        let next_position = direction.jump_cell(position);
        let (next_node_x, next_node_y) = next_position;
        let next_node = map.get_cell(next_position);
        // println!("Going from {:?} to {:?} with direction {:?}", position, next_node, direction);
        match next_node {
            Some(Node { letter: '#', visited: _ }) => {
                // If it's already a blocker, we don't have to run a loop test.
                traverse_map_for_loops(map, direction.rotate_90().unwrap(), position)
            }
            Some(_node) => {
                // This is where we could put a new obstruction to force us to go in circles
                let alternative_route = direction.clone().rotate_90().unwrap();
                
                let mut blocked_map = map.clone();
                blocked_map.cells[next_node_y as usize][next_node_x as usize] = {
                    Node { visited: false, letter: '#' }
                };
                let will_loop_if_blocked = traverse_will_loop(
                    &blocked_map,
                    alternative_route,
                    position,
                    &mut Vec::new()
                );
                let extra_loops = traverse_map_for_loops(map, direction, next_position);
                if will_loop_if_blocked {
                    vec![next_position].into_iter().chain(extra_loops.into_iter()).collect()
                } else {
                    extra_loops
                }
            }
            None => { Vec::new() }
        }
    }

    let mut looping_new_blockers = traverse_map_for_loops(&p2_map, Direction::N, start_position);

    let sum_visited_cells = map.cells.into_iter().fold(0, |total, row| {
        total +
            row
                .iter()
                .filter(|n| n.visited)
                .count()
    });

    looping_new_blockers.sort();
    println!("looping blockers {:?}", looping_new_blockers);
    looping_new_blockers.dedup();
    // TODO:: Part 2 answer is still wrong. giving up after hours of bashing my head against the wall.
    println!("Part 1 (sum of cells visited): {}", sum_visited_cells);
    println!("Part 2 (number of potential infinite loop blockers: {})", looping_new_blockers.len());
}

pub fn day7() {
    struct MaybeAnswerable {
        answer: i64,
        ordered_operatees: Vec<i64>
    }
    enum Operators {
        Add,
        Multiply
    }
    let maybe_answerables: Vec<MaybeAnswerable> = read_input_as_lines("day7_input.txt").into_iter().filter_map(|row| {
        let split_row = row.split_once(": ");
        if let Some((answer, operatees)) = split_row {
            Some(MaybeAnswerable {
                answer: answer.parse().unwrap(),
                ordered_operatees: operatees.split_whitespace().map(|operatee| operatee.parse::<i64>().unwrap()).collect()
            })
        } else {
            None
        }
    }).collect();

    // Recursive fold right to evaluate: The current index Plus or Times the calculated total of all indexes to its left, for each + and * multiple possible
    fn calculate(ordered_operatees: Vec<i64>, prev: i64) -> Vec<i64>  {
        let mut operatee_iterator = ordered_operatees.into_iter();
        // There should be no empty operatee lists for this initial state.
        let start = operatee_iterator.next();
        println!("Start {:?}", start);
        match start {
            Some(operatee) => {
                let downstream: Vec<i64> = calculate(operatee_iterator.collect(), operatee);
                println!("{:?} */+ downstream: {:?}", operatee, downstream);
                if !downstream.is_empty() {
                    downstream
                        .clone()
                        .iter()
                        .map(|item| operatee + item)
                        .chain(downstream.into_iter()
                            .map(|item| operatee * item)
                        )
                        .collect()
                } else {
                    vec![operatee]
                }
            },
            _ => {
                Vec::new()
            }
        }
    }
    let correct_answers = maybe_answerables.iter().filter(|maybe_answerable| {
        let answer_fold = maybe_answerable.ordered_operatees.clone().into_iter().rev().collect();
        println!("Processing {:?}", answer_fold);
        let potential_answers = calculate(answer_fold, 0);
        potential_answers.into_iter().any(|answer| answer == maybe_answerable.answer)
    });

    let answerable_total = correct_answers.fold(0, |total, maybe_answerable| total + maybe_answerable.answer);
    println!("Part 1 (Total from answerable rows): {}", answerable_total);
}