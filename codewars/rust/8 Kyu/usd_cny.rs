// Kata's link: https://www.codewars.com/kata/5977618080ef220766000022

fn usdcny(usd: u16) -> String {
    let ans = (usd as f32) * 6.75f32;
    format!("{ans:.2} Chinese Yuan")
}