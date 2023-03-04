// Kata's link: https://www.codewars.com/kata/525f50e3b73515a6db000b83

fn create_phone_number(numbers: &[u8]) -> String {
    format!("({}) {}-{}",
        numbers[..3].iter().map(|x| x.to_string()).collect::<String>(),
        numbers[3..6].iter().map(|x| x.to_string()).collect::<String>(),
        numbers[6..].iter().map(|x| x.to_string()).collect::<String>(),
    )
}