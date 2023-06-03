use std::{
    collections::{HashMap, HashSet},
    env, process,
};

use color_sort_solver::{Container, ContainerConstraint, State};

fn main() {
    let state = {
        let containers = match read_input() {
            Ok(input) => input,
            Err(errors) => {
                println!("Found following errors in input:");
                errors.iter().for_each(|e| println!("{e}"));
                process::exit(1);
            }
        };

        State::new(containers)
    };

    if state.is_solved() {
        println!("State is already solved!");
        return;
    }

    // -f == fast -- if fast flag is not found, then solve for optimal solution
    let optimal_solution = env::args().filter(|a| a == "-f").count() == 0;

    match state.solve(optimal_solution) {
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

/// conditions for input:
/// - All containers are same capacity
/// - Each color must be present number of times == container capacity
/// - # of colors <= # of containers
fn read_input() -> Result<Vec<Container>, Vec<String>> {
    let lines = {
        let mut vec = Vec::new();

        for line in std::io::stdin().lines() {
            match line {
                Ok(mut content) => {
                    content = content
                        .trim()
                        .to_owned()
                        .split(' ')
                        .filter(|s| *s != "") // Remove any additional spaces within line
                        .rev()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .to_uppercase();

                    if content != "" {
                        vec.push(content);
                    }
                }

                Err(err) => panic!("could not read stdin: {}", err),
            }
        }

        vec
    };

    let capacity: usize;

    // Validate the input
    {
        let mut errors = Vec::new();

        let all_colors: Vec<String> = {
            let mut colors = Vec::new();

            for line in lines.iter() {
                colors.extend(
                    line.split(' ')
                        .filter(|s| *s != "*" && *s != "-" && *s != "+")
                        .map(|s| s.to_owned()),
                );
            }

            colors
        };

        // Validate that all containers have same capacity
        {
            let lines_with_content = lines
                .iter()
                .filter(|s| *s != "*")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            if lines_with_content.is_empty() {
                return Ok(Vec::new());
            }

            let capacities: Vec<usize> = lines_with_content
                .iter()
                .map(|s| s.split(' ').filter(|s| *s != "+" && *s != "-").count())
                .collect();

            let first = capacities.first().unwrap().clone();
            let all_same = capacities.iter().all(|c| *c == first);
            if !all_same {
                errors.push(String::from("All containers must be of same capacity"));
            }

            capacity = first;
        }

        // Check that colors count <= containers count
        {
            let colors_set: HashSet<&String> = HashSet::from_iter(all_colors.iter());
            if colors_set.len() > lines.len() {
                errors.push(format!(
                    "colors count ({}) > containers count ({})",
                    colors_set.len(),
                    lines.len()
                ))
            }
        }

        // Check that # of each color == capacity of containers
        {
            let mut color_counts: HashMap<&String, i32> = HashMap::new();
            for color in all_colors.iter() {
                let count = color_counts.entry(color).or_default();
                *count += 1;
            }

            for entry in color_counts {
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

    let containers = {
        let mut color_ids = HashMap::new();
        let mut containers: Vec<Container> = Vec::new();

        for line in &lines {
            let mut content = Vec::new();
            let mut constraint = ContainerConstraint::None;

            for color in line.split(' ') {
                if color == "+" {
                    constraint = ContainerConstraint::MustFill;
                    continue;
                }

                if color == "-" {
                    constraint = ContainerConstraint::MustEmpty;
                    continue;
                }

                if color == "*" {
                    continue;
                }

                let next_id = color_ids.len() + 1;
                let color_id = color_ids.entry(color).or_insert(next_id);
                content.push(color_id.clone());
            }

            containers.push(Container::new(content, constraint, capacity).unwrap());
        }

        containers
    };

    return Ok(containers);
}
