// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread::spawn;

fn sum_half(s: &[i32]) -> i32 {
    s.iter().sum()
}

pub fn sum(v: Vec<i32>) -> i32 {
    let leaked_v = v.leak();
    let split_idx = leaked_v.len() / 2;
    let (first_half, second_half) = leaked_v.split_at(split_idx);
    let first_result = spawn(|| sum_half(first_half));
    let second_result = spawn(|| sum_half(second_half));
    first_result.join().unwrap() + second_result.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
