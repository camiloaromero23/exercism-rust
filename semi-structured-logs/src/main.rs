use semi_structured_logs::*;

fn main() {
    println!("{}", warn("AAAA"));
    println!("{}", info("BBBB"));
    println!("{}", error("CCCC"));
}
