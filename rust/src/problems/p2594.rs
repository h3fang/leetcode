pub struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut ranks = ranks.into_iter().map(|r| r as i64).collect::<Vec<_>>();
        ranks.sort_unstable_by_key(|r| -r);
        fn f(ranks: &[i64], t: i64) -> i64 {
            ranks.iter().map(|r| ((t / r) as f64).sqrt() as i64).sum()
        }
        let cars = cars as i64;
        let (mut left, mut right) = (1, *ranks.iter().max().unwrap() * cars * cars);
        while left < right {
            let m = (right - left) / 2 + left;
            if f(&ranks, m) < cars {
                left = m + 1;
            } else {
                right = m;
            }
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(16, Solution::repair_cars(vec![4, 2, 3, 1], 10));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::repair_cars(vec![5, 1, 8], 6));
    }
}
