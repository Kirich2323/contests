// Kata's link: https://www.codewars.com/kata/51c8991dee245d7ddf00000e

fn reverse_words(words: &str) -> String {
    words.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}