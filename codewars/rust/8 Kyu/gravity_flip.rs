// Kata's link: https://www.codewars.com/kata/5f70c883e10f9e0001c89673

fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut v = cubes.iter().copied().collect::<Vec<u32>>();
    if dir == 'R' {
        v.sort_by(|a, b| a.cmp(b))
    }
    else {
        v.sort_by(|a, b| b.cmp(a))
    }
    v
}