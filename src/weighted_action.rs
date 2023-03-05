use std::cmp::Ordering;
#[derive(Copy, Clone, Debug)]
pub struct WeightedAction<T: PartialEq> {
    pub action: T,
    pub weight: f32,
}

impl<T: PartialEq> WeightedAction<T> {
    pub fn new(action: T, weight: f32) -> Self {
        Self { action, weight }
    }
}

impl<T: PartialEq> Eq for WeightedAction<T> {}

impl<T: PartialEq> Ord for WeightedAction<T> {
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

impl<T: PartialEq> PartialOrd for WeightedAction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<T: PartialEq> PartialEq for WeightedAction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.action == other.action
    }
}
