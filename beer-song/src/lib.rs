pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} {1} of beer on the wall, {0} {1} of beer.\nTake one down and pass it around, {2} {3} of beer on the wall.\n",n,bottles_name(n),n-1,bottles_name(n-1))
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}

fn bottles_name(n: u32) -> &'static str {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
}
