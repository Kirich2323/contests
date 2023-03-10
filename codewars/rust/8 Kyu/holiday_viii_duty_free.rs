// Kata's link: https://www.codewars.com/kata/57e92e91b63b6cbac20001e5

fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    let savings = (price as f64) * (discount as f64 / 100.0);
    (holiday_cost as f64 / savings).floor() as i32
}