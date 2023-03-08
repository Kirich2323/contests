// Kata's link: https://www.codewars.com/kata/557cd6882bfa3c8a9f0000c1

fn get_age(age: &str) -> u32 {
    age.chars().nth(0).unwrap().to_digit(10).unwrap()
}