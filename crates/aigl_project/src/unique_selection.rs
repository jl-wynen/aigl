use rand::prelude::*;
use std::borrow::Borrow;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

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
    T: Clone + Eq,
{
    pub fn new<'c, 'u, Choices, Used, C, U>(
        choices: Choices,
        used: Used,
        fallback: fn(usize) -> T,
    ) -> Self
    where
        Choices: IntoIterator<Item = &'c C>,
        Used: IntoIterator<Item = U>,
        C: 'c + Eq + Hash + ToOwned<Owned = T> + ?Sized,
        U: 'u + Eq + Hash + Borrow<C>,
    {
        let used = used.into_iter().collect::<HashSet<_>>();
        // convert to HashSet to remove duplicates
        let mut choices = HashSet::<&C>::from_iter(choices)
            .into_iter()
            // custom filter for flexible comparison between types
            .filter(|c| !used.contains(c))
            .map(|c| c.to_owned())
            .collect::<VecDeque<_>>();

        let mut rng = rand::rng();
        choices.make_contiguous().shuffle(&mut rng);

        Self {
            choices,
            fallback,
            n_fallbacks: 0,
        }
    }

    pub fn pop(&mut self) -> T {
        self.choices.pop_back().unwrap_or_else(|| {
            self.n_fallbacks += 1;
            (self.fallback)(self.n_fallbacks)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unique_random_selection_produces_given_choices() {
        let choices = ["A", "B"];
        let used: [&str; 0] = [];
        let fallback = |i| format!("{i}");
        let mut selection = UniqueRandomSelection::new(choices, used, fallback);

        let drawn: HashSet<_> = (0..2).map(|_| selection.pop()).collect();
        assert_eq!(drawn, HashSet::from(["A".to_string(), "B".to_string()]));
    }

    #[test]
    fn unique_random_selection_produces_fallbacks() {
        let choices = ["A"];
        let used: [&str; 0] = [];
        let fallback = |i| format!("{i}");
        let mut selection = UniqueRandomSelection::new(choices, used, fallback);

        let _ = selection.pop(); // skip over given choice
        let drawn: HashSet<_> = (0..2).map(|_| selection.pop()).collect();
        assert_eq!(drawn, HashSet::from(["1".to_string(), "2".to_string()]));
    }

    #[test]
    fn unique_random_selection_removes_duplicates() {
        let choices = ["A", "B", "A"];
        let used: [&str; 0] = [];
        let fallback = |i| format!("{i}");
        let mut selection = UniqueRandomSelection::new(choices, used, fallback);

        let drawn: HashSet<_> = (0..3).map(|_| selection.pop()).collect();
        assert_eq!(
            drawn,
            HashSet::from(["A".to_string(), "B".to_string(), "1".to_string()])
        );
    }

    #[test]
    fn unique_random_selection_skips_used_values() {
        let choices = ["A", "B", "D", "E"];
        let used = ["E", "B"];
        let fallback = |i| format!("{i}");
        let mut selection = UniqueRandomSelection::new(choices, used, fallback);

        let drawn: HashSet<_> = (0..4).map(|_| selection.pop()).collect();
        assert_eq!(
            drawn,
            HashSet::from([
                "A".to_string(),
                "D".to_string(),
                "1".to_string(),
                "2".to_string()
            ])
        );
    }
}
