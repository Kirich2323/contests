// Kata's link: https://www.codewars.com/kata/563b662a59afc2b5120000c6

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut years = 0;
    let mut p2 = p0;
    while p2 < p {
        p2 += ((p2 as f64) * percent/100f64 + (aug as f64)) as i32;
        years += 1;
    };
    years
}