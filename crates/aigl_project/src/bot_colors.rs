use super::unique_selection::UniqueRandomSelection;
use std::borrow::Borrow;
use std::hash::Hash;

pub fn bot_color_selection<'u, Used, U>(used: Used) -> UniqueRandomSelection<String>
where
    Used: IntoIterator<Item = U>,
    U: 'u + Eq + Hash + Borrow<str>,
{
    UniqueRandomSelection::new(COLORS.iter().copied(), used, |i| {
        COLORS[i % COLORS.len()].to_string()
    })
}

// Generated with https://medialab.github.io/iwanthue/
const COLORS: [&str; 24] = [
    "#3a4cd5", "#9fda40", "#852990", "#02cb67", "#f14c96", "#2a862c", "#9b1f60", "#5bd8a4",
    "#fe3d44", "#42c4ec", "#a42035", "#2fb0ac", "#eb6b47", "#505591", "#d7c356", "#4f1a60",
    "#a5d387", "#2a5219", "#804e0e", "#9b8eed", "#f1a05b", "#d1be85", "#e87a86", "#eda3bc",
];
