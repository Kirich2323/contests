// Kata's link: https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec

fn do_the_trick(n: u64) -> u64 {
    n.to_string().chars().fold(1, |x, y| x * y.to_string().parse::<u64>().unwrap())
}

fn persistence(num: u64) -> u64 {
    let mut n = num;
    let mut ans = 0;
    while n.to_string().len() > 1 {
        n = do_the_trick(n);
        ans += 1;
    }
    ans
}