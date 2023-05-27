use std::{cmp::Ordering, hash::Hash};

pub struct WithScore<T, S> {
    value: T,
    score: S,
}

impl<T, S: Ord> WithScore<T, S> {
    pub fn new(value: T, score: S) -> Self {
        WithScore { value, score }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T, S: Ord> PartialEq for WithScore<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }

    fn ne(&self, other: &Self) -> bool {
        self.score != other.score
    }
}

impl<T, S: Ord> Eq for WithScore<T, S> {}

impl<T, S: Ord> PartialOrd for WithScore<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl<T, S: Ord> Ord for WithScore<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<T: Hash, S> Hash for WithScore<T, S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
