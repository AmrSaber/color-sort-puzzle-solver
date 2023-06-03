use std::hash::Hash;

#[derive(Clone)]
pub struct Container {
    content: Vec<usize>,
    capacity: usize,
    constraint: ContainerConstraint,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ContainerConstraint {
    MustFill,
    MustEmpty,
    None,
}

impl Container {
    pub fn new(
        content: Vec<usize>,
        constraint: ContainerConstraint,
        capacity: usize,
    ) -> Result<Self, String> {
        if content.len() > 0 && content.len() > capacity {
            return Err(format!(
                "content size ({}) exceeds capacity!",
                content.len()
            ));
        }

        return Ok(Self {
            content,
            capacity,
            constraint,
        });
    }

    fn peek(&self) -> Option<&usize> {
        return self.content.last();
    }

    pub fn is_empty(&self) -> bool {
        return self.content.is_empty();
    }

    pub fn is_full(&self) -> bool {
        return self.content.len() == self.capacity;
    }

    pub fn is_same_color(&self) -> bool {
        let first = self.content.first().unwrap();
        return self.content.iter().all(|color| color == first);
    }

    pub fn is_sorted(&self) -> bool {
        self.content.len() == self.capacity && self.is_same_color()
    }

    pub fn must_fill(&self) -> bool {
        self.constraint == ContainerConstraint::MustFill
    }

    pub fn must_empty(&self) -> bool {
        self.constraint == ContainerConstraint::MustEmpty
    }

    pub fn can_pour_into(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_full() {
            return false;
        }

        // This is disallowed because it's a no-op
        if self.is_same_color() && other.is_empty() {
            return false;
        }

        if other.is_empty() {
            return true;
        }

        return self.peek() == other.peek();
    }

    pub fn pour_into(&mut self, other: &mut Self) {
        if !self.can_pour_into(other) {
            panic!("cannot pour from {} into {}", self, other);
        }

        while self.can_pour_into(other) {
            let color = self.content.pop().unwrap();
            other.content.push(color);
        }
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Container({:?})", self.content)
    }
}

impl Hash for Container {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.content.iter().for_each(|e| e.hash(state));
    }
}
