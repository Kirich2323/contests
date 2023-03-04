//Kata's link: https://www.codewars.com/kata/52597aa56021e91c93000cb0

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut v = arr.iter().cloned().filter(|&x| x != 0u8).collect::<Vec<_>>();
    v.append(&mut (vec![0; arr.len() - v.len()]));
    v
}