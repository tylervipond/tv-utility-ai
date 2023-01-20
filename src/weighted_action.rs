use std::cmp::Ordering;
#[derive(Copy, Clone)]
pub struct WeightedAction<T> {
    pub action: T,
    pub weight: f32,
}

impl<T> WeightedAction<T> {
    pub fn new(action: T, weight: f32) -> Self {
        Self { action, weight }
    }
}

impl<T> Eq for WeightedAction<T> {}

impl<T> Ord for WeightedAction<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight > other.weight {
            return Ordering::Greater;
        }
        if self.weight < other.weight {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl<T> PartialOrd for WeightedAction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<T> PartialEq for WeightedAction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
