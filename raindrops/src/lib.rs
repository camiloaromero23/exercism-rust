pub fn raindrops(n: u32) -> String {
    let factors = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let drops = factors.iter().fold("".to_owned(), |mut sound, &(f, v)| {
        if n % f == 0 {
            sound.push_str(v);
        }
        sound
    });

    if drops.is_empty() {
        return n.to_string();
    }
    drops
}
