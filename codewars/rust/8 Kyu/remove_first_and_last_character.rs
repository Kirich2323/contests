// Kata's link: https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0

pub fn remove_char(s: &str) -> String {
    s[1..s.len()-1].to_string()
}