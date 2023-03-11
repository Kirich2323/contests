// Kata's link: https://www.codewars.com/kata/576bb71bbbcf0951d5000044

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() > 0 {
        let p = input.iter().map(|x: &i32| (*x > 0) as i32).sum();
        let s = input.iter().filter(|&x| (*x < 0)).sum();
        [p, s].to_vec()
    }
    else {
        [].to_vec()
    }
}