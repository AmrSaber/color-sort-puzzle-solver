#[derive(Clone)]
pub struct Transition {
    pour_from: usize,
    pour_into: usize,
}

impl Transition {
    pub fn new(from: usize, to: usize) -> Self {
        return Self {
            pour_from: from,
            pour_into: to,
        };
    }

    pub fn pour_from(&self) -> usize {
        self.pour_from
    }

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
