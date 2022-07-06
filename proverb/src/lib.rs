use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    // if list.is_empty() {
    //     return String::new();
    // }

    // let mut proverb = String::new();

    // for n in 0..list.len() - 1 {
    //     proverb.push_str(&format!(
    //         "For want of a {} the {} was lost.\n",
    //         &list[n],
    //         &list[n + 1]
    //     ));
    // }
    // proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    // proverb
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}
