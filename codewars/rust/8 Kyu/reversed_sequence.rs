// Kata's link: https://www.codewars.com/kata/5a00e05cc374cb34d100000d

fn reverse_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}