mod models;
use std::{
    collections::{HashMap, HashSet},
    process,
};

use models::{State, CONTAINER_CAPACITY, CONTAINER_DASH_ID, CONTAINER_STAR_ID};

fn main() {
    let state = {
        let lines = match read_input() {
            Ok(input) => input,
            Err(errors) => {
                println!("Found following errors in input:");
                errors.iter().for_each(|e| println!("{e}"));
                process::exit(1);
            }
        };

        match State::from_strings(lines) {
            Ok(state) => state,
            Err(err) => {
                println!("error: {err}");
                process::exit(1);
            }
        }
    };

    if state.is_solved() {
        println!("state is already solved!");
        return;
    }

    match state.solve() {
        Some(solved) => {
            println!(
                "Found solution in {} steps:",
                solved.get_transitions().len()
            );

            solved
                .get_transitions()
                .iter()
                .for_each(|t| println!("- {}", t));
        }
        None => println!("No solution!"),
    }
}

fn read_input() -> Result<Vec<Vec<isize>>, Vec<String>> {
    let mut lines = Vec::new();

    for line in std::io::stdin().lines() {
        match line {
            Ok(mut content) => {
                content = content
                    .trim()
                    .to_owned()
                    .split(' ')
                    .rev()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .to_uppercase();

                if content != "" {
                    lines.push(content);
                }
            }

            Err(err) => panic!("could not read stdin: {}", err),
        }
    }

    // Validate the input
    {
        let mut errors = Vec::new();

        let all_colors: Vec<String> = {
            let mut colors = Vec::new();

            for line in lines.iter() {
                colors.extend(line.split(' ').map(|s| s.to_owned()));
            }

            colors
                .into_iter()
                .filter(|s| s != "*" && s != "-")
                .collect()
        };

        // Check that colors count == containers count
        {
            let colors_set: HashSet<&String> = HashSet::from_iter(all_colors.iter());
            if colors_set.len() != lines.len() {
                errors.push(format!(
                    "colors count ({}) != containers count ({})",
                    colors_set.len(),
                    lines.len()
                ))
            }
        }

        // Check that each color exists MAX_SIZE times
        {
            let mut colors_map: HashMap<&String, i32> = HashMap::new();
            for color in all_colors.iter() {
                let entry = colors_map.entry(color).or_default();
                *entry += 1;
            }

            for entry in colors_map {
                if entry.1 != CONTAINER_CAPACITY as i32 {
                    errors.push(format!(
                        "color \"{}\" only exists {} time(s) not {}",
                        entry.0, entry.1, CONTAINER_CAPACITY
                    ))
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
    }

    // Transform colors into numbers
    let input: Vec<Vec<isize>> = {
        let mut color_ids = HashMap::new();
        color_ids.insert("*", CONTAINER_STAR_ID);
        color_ids.insert("-", CONTAINER_DASH_ID);

        for line in &lines {
            for color in line.split(' ') {
                if color == "" {
                    continue;
                }

                let default_id = color_ids.len() as isize + 1;
                color_ids.entry(color).or_insert(default_id);
            }
        }

        lines
            .iter()
            .map(|line| {
                line.split(' ')
                    .filter(|c| *c != "")
                    .map(|c| color_ids.get(c).unwrap().clone())
                    .collect()
            })
            .collect()
    };

    return Ok(input);
}
