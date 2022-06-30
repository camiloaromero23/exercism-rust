pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let number_of_digits: u32 = digits.len() as u32;

    let sum = digits.chars().fold(0, |mut sum, digit| {
        sum += digit
            .to_string()
            .parse::<u32>()
            .unwrap()
            .pow(number_of_digits);
        sum
    });

    sum == num
}
