use super::unique_selection::UniqueRandomSelection;
use std::borrow::Borrow;
use std::hash::Hash;

pub fn bot_name_selection<'u, Used, U>(used: Used) -> UniqueRandomSelection<String>
where
    Used: IntoIterator<Item = U>,
    U: 'u + Eq + Hash + Borrow<str>,
{
    UniqueRandomSelection::new(NAMES.iter().copied(), used, |i| format!("Bot{i}"))
}

const NAMES: [&str; 18] = [
    "Botulf",
    "Bottom",
    "Boten",
    "Botholomew",
    "Botill",
    "Lisbot",
    "Aimbot",
    "Botbara",
    "Bothany",
    "Elisabot",
    "Charbotte",
    "Botlivia",
    "Isabotta",
    "Boteo",
    "Chrisbotpher",
    "Abotham",
    "Botacus",
    "Botcates",
];
