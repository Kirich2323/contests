// Kata's link: https://www.codewars.com/kata/5861d28f124b35723e00005e

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    if gallons * mpg >= distance_to_pump {
        true
    }
    else {
        false
    }
}