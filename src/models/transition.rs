/// A transition represents an action that is done on a state.
/// In this case, it's an action of pouring the content of one container into another container.
/// Thus a transition only holds 2 fields (pour_from, pour_into) representing the source and destination.
#[derive(Clone)]
pub struct Transition {
    pour_from: usize,
    pour_into: usize,
}

impl Transition {
    /// Create a new transition
    pub fn new(from: usize, to: usize) -> Self {
        return Self {
            pour_from: from,
            pour_into: to,
        };
    }

    /// The index of the container to pour from
    pub fn pour_from(&self) -> usize {
        self.pour_from
    }

    /// The index of the container to pour into
    pub fn pour_into(&self) -> usize {
        self.pour_into
    }
}

impl std::fmt::Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:02}) -> ({:02})",
            self.pour_from + 1,
            self.pour_into + 1
        )
    }
}
