// Kata's link: https://www.codewars.com/kata/58cb43f4256836ed95000f97

fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    i32::abs(a[0]*a[1]*a[2] - b[0]*b[1]*b[2])
}