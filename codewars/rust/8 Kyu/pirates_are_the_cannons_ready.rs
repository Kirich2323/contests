// Kata's link: https://www.codewars.com/kata/5748a883eb737cab000022a6

use std::collections::HashMap;

fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    if gunners.values().map(|x| (*x=="aye") as i32).sum::<i32>() == gunners.len() as i32 {
        "Fire!".to_string()
        } 
    else { 
        "Shiver me timbers!".to_string()
    }
}