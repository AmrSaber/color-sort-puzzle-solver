use super::{transition::Transition, Container};
use std::{cmp::Ordering, collections::BinaryHeap, collections::HashSet, hash::Hash, rc::Rc};

#[derive(Clone)]
pub struct State {
    containers: Vec<Container>,
    transitions: Vec<Transition>,
}

impl State {
    pub fn new(containers: Vec<Container>) -> Self {
        return Self {
            containers,
            transitions: Vec::new(),
        };
    }

    pub fn from_strings(lines: Vec<String>) -> Self {
        let mut containers: Vec<Container> = lines
            .into_iter()
            .map(|s| Container::from_string(s))
            .collect();

        // Add 2 empty containers
        containers.push(Container::new(Vec::new()));
        containers.push(Container::new(Vec::new()));

        return Self::new(containers);
    }

    pub fn get_transitions(&self) -> &Vec<Transition> {
        return &self.transitions;
    }

    fn sorted_count(&self) -> i32 {
        self.containers.iter().filter(|c| c.is_sorted()).count() as i32
    }

    fn empty_count(&self) -> i32 {
        self.containers.iter().filter(|c| c.is_empty()).count() as i32
    }

    fn stars_count(&self) -> i32 {
        self.containers
            .iter()
            .filter(|c| c.has_star() && c.is_sorted())
            .count() as i32
    }

    // The more the better
    fn get_score(&self) -> (i32, i32, i32) {
        return (
            self.sorted_count(),
            -(self.transitions.len() as i32),
            self.stars_count(),
        );
    }

    pub fn is_solved(&self) -> bool {
        let got_stars = self
            .containers
            .iter()
            .all(|c| !(c.has_star() && c.is_empty()));

        let all_sorted = self.empty_count() + self.sorted_count() == self.containers.len() as i32;

        return got_stars && all_sorted;
    }

    fn get_possible_transitions(&self) -> Vec<Transition> {
        let mut transitions = Vec::new();

        self.containers
            .iter()
            .enumerate()
            .for_each(|(from_index, from_container)| {
                self.containers
                    .iter()
                    .enumerate()
                    .for_each(|(to_index, to_container)| {
                        if from_index == to_index {
                            return;
                        }

                        if from_container.can_pour_into(to_container) {
                            transitions.push(Transition::new(from_index, to_index));
                        }
                    });
            });

        return transitions;
    }

    fn apply_transition(&self, transition: Transition) -> Self {
        let mut new_state = self.clone();

        unsafe {
            let from_container: *mut Container = &mut new_state.containers[transition.pour_from()];
            let to_container: *mut Container = &mut new_state.containers[transition.pour_into()];

            (*from_container).pour_into(&mut *to_container);
        }

        new_state.transitions.push(transition);

        return new_state;
    }

    fn get_possible_states(&self) -> Vec<State> {
        self.get_possible_transitions()
            .into_iter()
            .map(|t| self.apply_transition(t))
            .collect()
    }

    pub fn solve(&self) -> Option<Self> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        queue.push(Rc::new(self.clone()));

        while !queue.is_empty() {
            let top = queue.pop().unwrap();
            if top.is_solved() {
                let top = (*top).clone();
                return Some(top);
            }

            visited.insert(Rc::clone(&top));

            let possible_states = top.get_possible_states();
            possible_states.into_iter().for_each(|state| {
                let state = Rc::from(state);
                if !visited.contains(&state) {
                    queue.push(Rc::clone(&state));
                }
            });
        }

        return None;
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.get_score() == other.get_score()
    }

    fn ne(&self, other: &Self) -> bool {
        self.get_score() != other.get_score()
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_score().cmp(&other.get_score())
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let sorted_containers = {
            let mut containers: Vec<String> =
                self.containers.iter().map(|c| c.to_string()).collect();
            containers.sort_unstable();
            containers
        };

        for container in sorted_containers {
            container.hash(state);
        }
    }
}
