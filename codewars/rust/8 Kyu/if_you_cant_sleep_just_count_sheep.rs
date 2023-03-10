// Kata's link: https://www.codewars.com/kata/5b077ebdaf15be5c7f000077

fn count_sheep(n: u32) -> String {
    (1..=n).fold("".to_string(), |acc, x| acc + &format!("{x} sheep..."))
}