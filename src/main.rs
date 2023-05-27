use std::{
    collections::{HashMap, HashSet},
    process,
};

use color_sort_solver::{State, CONTAINER_DASH_ID, CONTAINER_STAR_ID};

fn main() {
    let state = {
        let (lines, capacity) = match read_input() {
            Ok(input) => input,
            Err(errors) => {
                println!("Found following errors in input:");
                errors.iter().for_each(|e| println!("{e}"));
                process::exit(1);
            }
        };

        match State::new(lines, capacity) {
            Ok(state) => state,
            Err(err) => {
                println!("error: {err}");
                process::exit(1);
            }
        }
    };

    if state.is_solved() {
        println!("State is already solved!");
        return;
    }

    match state.solve() {
        Some(solved) => {
            println!("Found solution in {} steps:", solved.transitions().len());

            solved
                .transitions()
                .iter()
                .for_each(|t| println!("- {}", t));
        }
        None => println!("No solution!"),
    }
}

fn read_input() -> Result<(Vec<Vec<isize>>, usize), Vec<String>> {
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

    // Figure out the capacity based on max line size of input
    let capacity = {
        let mut cap = 0;

        for line in &lines {
            let length = line
                .replace("*", "")
                .replace("-", "")
                .trim()
                .split(" ")
                .filter(|s| *s != "")
                .count();

            cap = usize::max(cap, length);
        }

        cap
    };

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
                if entry.1 != capacity as i32 {
                    errors.push(format!(
                        "color \"{}\" only exists {} time(s) not {}",
                        entry.0, entry.1, capacity
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

    return Ok((input, capacity));
}
