mod models;
use std::collections::{HashMap, HashSet};

use models::{State, MAX_CONTAINER_SIZE};

fn main() {
    let state = {
        let lines = read_input();
        State::from_strings(lines)
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

fn read_input() -> Vec<String> {
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

            colors.into_iter().filter(|s| s != "*").collect()
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
                if entry.1 != MAX_CONTAINER_SIZE as i32 {
                    errors.push(format!(
                        "color {} only exists {} times not {}",
                        entry.0, entry.1, MAX_CONTAINER_SIZE
                    ))
                }
            }
        }

        if !errors.is_empty() {
            println!("Found following errors in input:");
            errors.iter().for_each(|e| println!("{e}"));
            panic!("invalid input")
        }
    }

    return lines;
}
