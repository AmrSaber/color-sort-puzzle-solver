use super::{transition::Transition, with_score::WithScore, Container};
use std::{collections::BinaryHeap, collections::HashSet, hash::Hash, rc::Rc};

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

    // The higher the better
    fn get_score(&self) -> (i32, i32, i32) {
        return (
            -(self.transitions.len() as i32),
            self.sorted_count(),
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

        let score = self.get_score();
        let current = Rc::new(WithScore::new(self.clone(), score));

        queue.push(Rc::clone(&current));
        visited.insert(Rc::clone(&current));

        while !queue.is_empty() {
            let top = queue.pop().unwrap();
            let current = top.value();

            if current.is_solved() {
                let state = (*current).clone();
                return Some(state);
            }

            let possible_states = current.get_possible_states();
            possible_states.into_iter().for_each(|state| {
                let score = state.get_score();
                let new = Rc::from(WithScore::new(state, score));

                if !visited.contains(&new) {
                    queue.push(Rc::clone(&new));
                    visited.insert(Rc::clone(&new));
                }
            });
        }

        return None;
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
