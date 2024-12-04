use std::{ collections::HashMap, fs };

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
