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
    /// Creates a new state (game) with the given containers. Empty containers must be provided as well.
    pub fn new(containers: Vec<Container>) -> Self {
        return Self {
            containers,
            transitions: Vec::new(),
        };
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

    fn must_fill_count(&self) -> i32 {
        self.containers
            .iter()
            .filter(|c| c.must_fill() && c.is_sorted())
            .count() as i32
    }

    fn must_empty_count(&self) -> i32 {
        self.containers
            .iter()
            .filter(|c| c.must_empty() && c.is_empty())
            .count() as i32
    }

    /// Score of the current state, the higher the better.
    /// For optimal solution, only distance (current + heuristic) is used as a form of A* search
    /// For suboptimal solution, the number of sorted containers is the priority as a form of greedy search
    fn get_score(&self, optimal: bool) -> impl Ord {
        let remaining = self.containers.len() as i32 - self.sorted_count();
        let distance = self.transitions.len() as i32;

        let heuristic_distance = distance + remaining;

        if optimal {
            return (
                -heuristic_distance,
                self.must_empty_count(),
                self.must_fill_count(),
                0,
            );
        }

        return (
            self.sorted_count(),
            -heuristic_distance,
            self.must_empty_count(),
            self.must_fill_count(),
        );
    }

    /// Check if the state is solved or not
    pub fn is_solved(&self) -> bool {
        let got_fills = self
            .containers
            .iter()
            .all(|c| !(c.must_fill() && c.is_empty()));

        let got_empties = self
            .containers
            .iter()
            .all(|c| !(c.must_empty() && !c.is_empty()));

        let all_sorted = self.empty_count() + self.sorted_count() == self.containers.len() as i32;

        return all_sorted && got_fills && got_empties;
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
    /// Accepts a boolean to set whether or not get the optimal solution.
    pub fn solve(&self, optimal: bool) -> Option<Self> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        let score = self.get_score(optimal);
        let current = Rc::new(WithScore::new(self.clone(), score));

        queue.push(Rc::clone(&current));
        visited.insert(Rc::clone(&current));

        while let Some(top) = queue.pop() {
            let current = top.value();

            if current.is_solved() {
                let state = (*current).clone();
                return Some(state);
            }

            for state in current.get_possible_states() {
                let score = state.get_score(optimal);
                let new = Rc::from(WithScore::new(state, score));

                if !visited.contains(&new) {
                    queue.push(Rc::clone(&new));
                    visited.insert(Rc::clone(&new));
                }
            }
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
