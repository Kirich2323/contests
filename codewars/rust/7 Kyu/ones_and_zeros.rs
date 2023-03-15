// Kata's link: https://www.codewars.com/kata/578553c3a1b8d5c40300037c

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let mut ans = 0u32;
    let mut pow = 0u32;
    for v in slice.iter().rev() {
        ans += v * u32::pow(2, pow);
        pow += 1;
    }
    ans
}