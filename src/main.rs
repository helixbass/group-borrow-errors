use std::{collections::HashSet, io};

use regex::Regex;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum CurrentState {
    #[default]
    Looking,
    SawMarkerLine,
    SawDashedLine,
}

#[derive(Hash, PartialEq, Eq)]
struct SeenError {
    file: String,
    line: usize,
    column: usize,
}

fn main() {
    let marker_line_regex = Regex::new(r#"^current ac"#).unwrap();
    let borrow_error_line_regex =
        Regex::new(r#"^Location \{ file: "([^"]+)", line: (\d+), col: (\d+) }"#).unwrap();
    let mut current_state = CurrentState::default();
    let mut seen_errors: HashSet<SeenError> = Default::default();
    let mut just_printed_blank_line = false;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        match current_state {
            CurrentState::Looking => {
                if marker_line_regex.is_match(&line) {
                    current_state = CurrentState::SawMarkerLine;
                }
            }
            CurrentState::SawMarkerLine => {
                if line.starts_with("---") {
                    current_state = CurrentState::SawDashedLine;
                } else {
                    assert!(line.trim().is_empty());
                    current_state = CurrentState::Looking;
                    if !just_printed_blank_line {
                        println!("");
                        just_printed_blank_line = true;
                    }
                }
            }
            CurrentState::SawDashedLine => {
                let captures = borrow_error_line_regex.captures(&line).unwrap();
                let error = SeenError {
                    file: captures[1].to_owned(),
                    line: captures[2].parse::<usize>().unwrap(),
                    column: captures[3].parse::<usize>().unwrap(),
                };
                if !seen_errors.contains(&error) {
                    println!("{line}");
                    just_printed_blank_line = false;
                    seen_errors.insert(error);
                }
                current_state = CurrentState::SawMarkerLine;
            }
        }
    }
}
