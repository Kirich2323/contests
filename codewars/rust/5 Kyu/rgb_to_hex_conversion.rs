// Kata's link: https://www.codewars.com/kata/513e08acc600c94f01000001

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", r.clamp(0, 255), g.clamp(0, 255), b.clamp(0, 255))
}