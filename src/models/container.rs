use std::hash::Hash;

#[derive(Clone)]
pub struct Container {
    content: Vec<usize>,
    capacity: usize,
    has_star: bool,
    has_dash: bool,
}

/// The number to be used in order to indicate that this container has a star
pub const CONTAINER_STAR_ID: isize = -1;

/// The number to be used in order to indicate that this container has a dash
pub const CONTAINER_DASH_ID: isize = -2;

impl Container {
    pub fn new(content: Vec<isize>, capacity: usize) -> Result<Self, String> {
        let has_dash = content.iter().any(|s| *s == CONTAINER_DASH_ID);
        let has_star = content.iter().any(|s| *s == CONTAINER_STAR_ID);

        if has_dash && has_star {
            return Err(String::from(
                "container cannot have dash and star at the same time!",
            ));
        }

        let content: Vec<usize> = content
            .into_iter()
            .filter(|s| *s > 0)
            .map(|e| e as usize)
            .collect();

        if content.len() > 0 && content.len() != capacity {
            return Err(format!(
                "content size ({}) does not match capacity!",
                content.len()
            ));
        }

        return Ok(Self {
            content,
            capacity,
            has_star,
            has_dash,
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

    pub fn is_sorted(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        let first = self.content.first().unwrap();
        return self.content.len() == self.capacity
            && self.content.iter().all(|color| color == first);
    }

    pub fn has_star(&self) -> bool {
        self.has_star
    }

    pub fn has_dash(&self) -> bool {
        self.has_dash
    }

    pub fn can_pour_into(&self, other: &Self) -> bool {
        if self.is_empty() || self.is_sorted() || other.is_full() {
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

    pub fn to_string(&self) -> String {
        self.content
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("|")
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
