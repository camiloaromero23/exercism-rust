use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut notes = HashMap::new();

    magazine.iter().for_each(|&w| {
        notes.entry(w).or_insert(0).add_assign(1);
    });

    note.iter().for_each(|&w| {
        notes.entry(w).or_insert(0).sub_assign(1);
    });

    notes.values().all(|&count| count >= 0)
}
