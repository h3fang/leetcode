use std::collections::BinaryHeap;

pub struct Solution;

#[derive(PartialEq, Eq)]
struct Fraction(i32, i32, usize, usize);

impl std::cmp::Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.0 * self.1).cmp(&(self.0 * other.1))
    }
}

impl std::cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut q = BinaryHeap::new();
        for (i, &a) in arr.iter().enumerate().skip(1) {
            q.push(Fraction(1, a, 0, i));
        }
        for _ in 1..k {
            let f = q.pop().unwrap();
            if f.2 + 1 < f.3 {
                q.push(Fraction(arr[f.2 + 1], f.1, f.2 + 1, f.3));
            }
        }
        let f = q.peek().unwrap();
        vec![f.0, f.1]
    }

    pub fn kth_smallest_prime_fraction_binary(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut left = 0.0;
        let mut right = 1.0;
        while left < right {
            let mid = (left + right) / 2.0;
            let mut i = -1;
            let mut count = 0;
            let mut x = 0;
            let mut y = 1;
            for j in 1..arr.len() {
                while (arr[(i + 1) as usize] as f64 / arr[j] as f64) < mid {
                    i += 1;
                    if x * arr[j] < y * arr[i as usize] {
                        x = arr[i as usize];
                        y = arr[j];
                    }
                }
                count += (i + 1) as usize;
            }

            match count.cmp(&k) {
                std::cmp::Ordering::Less => left = mid,
                std::cmp::Ordering::Equal => return vec![x, y],
                std::cmp::Ordering::Greater => right = mid,
            }
        }
        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        assert_eq!(
            vec![2, 5],
            Solution::kth_smallest_prime_fraction_binary(arr.clone(), k)
        );
        assert_eq!(vec![2, 5], Solution::kth_smallest_prime_fraction(arr, k));
    }
}
