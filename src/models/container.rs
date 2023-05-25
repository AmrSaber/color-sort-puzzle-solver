use std::hash::Hash;

#[derive(Clone)]
pub struct Container {
    content: Vec<String>,
    has_star: bool,
}

pub const MAX_CONTAINER_SIZE: usize = 4;

impl Container {
    pub fn new(mut content: Vec<String>) -> Self {
        let has_star = content.iter().any(|s| s == "*");
        content = content.into_iter().filter(|s| s != "*").collect();

        if content.len() > MAX_CONTAINER_SIZE {
            panic!("content ({:?}) too large!", content);
        }

        return Self { content, has_star };
    }

    pub fn from_string(line: String) -> Self {
        let content: Vec<String> = line.split(" ").map(|s| s.to_owned()).collect();
        return Self::new(content);
    }

    fn peek(&self) -> Option<&String> {
        return self.content.last();
    }

    pub fn is_empty(&self) -> bool {
        return self.content.is_empty();
    }

    pub fn is_full(&self) -> bool {
        return self.content.len() == MAX_CONTAINER_SIZE;
    }

    pub fn is_sorted(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        let first = self.content.first().unwrap();
        return self.content.len() == MAX_CONTAINER_SIZE
            && self.content.iter().all(|color| color == first);
    }

    pub fn has_star(&self) -> bool {
        self.has_star
    }

    pub fn can_pour_into(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_full() {
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
        self.content.join("|")
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Container({:?})", self.content)
    }
}

impl Hash for Container {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.content.iter().for_each(|s| s.hash(state));
    }
}
