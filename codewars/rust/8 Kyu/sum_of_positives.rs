// Kata's link: https://www.codewars.com/kata/5715eaedb436cf5606000381

fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    slice.iter().filter(|x| **x > 0).sum()
}