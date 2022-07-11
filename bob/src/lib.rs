fn is_asking(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    let has_letters = message.contains(char::is_alphabetic);

    has_letters && message.to_uppercase() == message
}

fn says_nothing(message: &str) -> bool {
    message.is_empty()
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if is_asking(m) && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if is_asking(m) => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        m if says_nothing(m) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
