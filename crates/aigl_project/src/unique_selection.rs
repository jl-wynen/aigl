use rand::prelude::*;
use std::collections::{HashSet, VecDeque};

/// A collection of unique, randomly sampled values from a collection.
///
/// Should be initialized with the full list of choices
/// and then used through `pop`.
/// `push` should only be used to put back values that were taken out with `pop`.
pub struct UniqueRandomSelection<T> {
    choices: VecDeque<T>,
    fallback: fn(usize) -> T,
    n_fallbacks: usize,
}

impl<T> UniqueRandomSelection<T>
where
    T: Eq + std::hash::Hash,
{
    fn new<C: IntoIterator<Item = T>>(choices: C, fallback: fn(usize) -> T) -> Self {
        let mut choices = choices
            .into_iter()
            .collect::<HashSet<_>>() // remove duplicates
            .into_iter()
            .collect::<VecDeque<_>>();

        let mut rng = rand::rng();
        choices.make_contiguous().shuffle(&mut rng);

        Self {
            choices,
            fallback,
            n_fallbacks: 0,
        }
    }

    fn pop(&mut self) -> T {
        self.choices.pop_back().unwrap_or_else(|| {
            self.n_fallbacks += 1;
            (self.fallback)(self.n_fallbacks)
        })
    }

    fn push(&mut self, value: T) {
        self.choices.push_front(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unique_random_selection_produces_given_choices() {
        let mut selection =
            UniqueRandomSelection::new(["A".to_string(), "B".to_string()], |i| format!("{i}"));
        let drawn: HashSet<_> = (0..2).map(|_| selection.pop()).collect();
        assert_eq!(drawn, HashSet::from(["A".to_string(), "B".to_string()]));
    }

    #[test]
    fn unique_random_selection_produces_fallbacks() {
        let mut selection = UniqueRandomSelection::new(["A".to_string()], |i| format!("{i}"));
        let _ = selection.pop(); // skip over given choice

        let drawn: HashSet<_> = (0..2).map(|_| selection.pop()).collect();
        assert_eq!(drawn, HashSet::from(["1".to_string(), "2".to_string()]));
    }

    #[test]
    fn unique_random_selection_choices_reappear_after_being_put_back() {
        let mut selection =
            UniqueRandomSelection::new(["A".to_string(), "B".to_string()], |i| format!("{i}"));
        let a = selection.pop();
        selection.push(a);
        let drawn: HashSet<_> = (0..2).map(|_| selection.pop()).collect();
        assert_eq!(drawn, HashSet::from(["A".to_string(), "B".to_string()]));
    }

    #[test]
    fn unique_random_selection_removes_duplicates() {
        let mut selection =
            UniqueRandomSelection::new(["A".to_string(), "B".to_string(), "A".to_string()], |i| {
                format!("{i}")
            });
        let drawn: HashSet<_> = (0..3).map(|_| selection.pop()).collect();
        assert_eq!(
            drawn,
            HashSet::from(["A".to_string(), "B".to_string(), "1".to_string()])
        );
    }
}
