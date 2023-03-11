// Kata's link: https://www.codewars.com/kata/58f8a3a27a5c28d92e000144

fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    for i in 0..arr.len()-1 {
        if arr[i+1] - arr[i] != 1 {
            return Some(arr[i+1]);
        }
    }
    None
}