// Kata's link: https://www.codewars.com/kata/5583090cbe83f4fd8c000051

fn digitize(n: u64) -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    let mut new_n = n;
    
    while new_n > 0 {
        v.push( (new_n % 10) as u8);
        new_n /= 10;
    }
    if v.is_empty() {
        vec![0]
    }
    else {
        v    
    }
}