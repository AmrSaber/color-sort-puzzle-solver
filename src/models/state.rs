use super::{Container, Transition, WithScore};
use std::{collections::BinaryHeap, collections::HashSet, hash::Hash, rc::Rc};

/// State represents the state of a set of containers.
/// State is totally immutable. A new state is created when applying a transition.
#[derive(Clone)]
pub struct State {
    containers: Vec<Container>,
    transitions: Vec<Transition>,
}

impl State {
    /// Creates a new state (game) with the given content and capacity.
    /// Any value that is not +ve or one of the defined container IDs is ignored.
    /// Each of the inner vectors of the content MUST contain the exact same count of +ve numbers as the capacity.
    pub fn new(content: Vec<Vec<isize>>, capacity: usize) -> Result<Self, String> {
        let mut containers: Vec<Container> = Vec::new();

        for line in content {
            let container = match Container::new(line, capacity) {
                Ok(container) => container,
                Err(err) => return Err(err),
            };

            containers.push(container);
        }

        // Add 2 empty containers
        containers.push(Container::new(Vec::new(), capacity).unwrap());
        containers.push(Container::new(Vec::new(), capacity).unwrap());

        let dash_count = containers.iter().filter(|c| c.has_dash()).count();
        if dash_count > 2 {
            return Err(String::from(
                "cannot mark more than 2 containers to be empty!",
            ));
        }

        return Ok(Self {
            containers,
            transitions: Vec::new(),
        });
    }

    /// Get transitions that lead to this state from the original state.
    pub fn transitions(&self) -> &Vec<Transition> {
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

    fn dashes_count(&self) -> i32 {
        self.containers
            .iter()
            .filter(|c| c.has_dash() && c.is_empty())
            .count() as i32
    }

    // The higher the better
    fn get_score(&self) -> (i32, i32, i32, i32) {
        return (
            -(self.transitions.len() as i32),
            self.sorted_count(),
            self.stars_count(),
            self.dashes_count(),
        );
    }

    /// Check if the state is solved or not
    pub fn is_solved(&self) -> bool {
        let got_stars = self
            .containers
            .iter()
            .all(|c| !(c.has_star() && c.is_empty()));

        let got_dashes = self
            .containers
            .iter()
            .all(|c: &Container| !(c.has_dash() && !c.is_empty()));

        let all_sorted = self.empty_count() + self.sorted_count() == self.containers.len() as i32;

        return all_sorted && got_stars && got_dashes;
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

    /// Returns a solved state if there is a solution, and None otherwise.
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
        // Hash the containers in a sorted order
        // this eliminates similar states with different order of containers
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
