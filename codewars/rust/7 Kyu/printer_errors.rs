// Kata's link: https://www.codewars.com/kata/56541980fa08ab47a0000040

fn printer_error(s: &str) -> String {
    format!("{}/{}", s.chars().map(|x| (x > 'm') as i32).sum::<i32>(), s.len())
}