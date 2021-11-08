pub struct Solution;

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        fn can_distribute(x: i32, n: i32, quantities: &[i32]) -> bool {
            let m: i32 = quantities.iter().map(|q| (q + x - 1) / x).sum();
            m <= n
        }

        if quantities.len() == n as usize {
            *quantities.iter().max().unwrap()
        } else {
            let mut left = 1;
            let mut right = *quantities.iter().max().unwrap();
            while left <= right {
                let mid = left + (right - left) / 2;
                if can_distribute(mid, n, &quantities) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 6;
        let quantities = vec![11, 6];
        assert_eq!(3, Solution::minimized_maximum(n, quantities));
    }

    #[test]
    fn case2() {
        let n = 1;
        let quantities = vec![100000];
        assert_eq!(100000, Solution::minimized_maximum(n, quantities));
    }

    #[test]
    fn case3() {
        let n = 7;
        let quantities = vec![15, 10, 10];
        assert_eq!(5, Solution::minimized_maximum(n, quantities));
    }
}
