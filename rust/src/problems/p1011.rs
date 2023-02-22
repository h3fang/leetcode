pub struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        fn check(weights: &[i32], m: i32) -> i32 {
            weights
                .iter()
                .fold((1, 0), |(days, curr), &w| {
                    if curr + w > m {
                        (days + 1, w)
                    } else {
                        (days, curr + w)
                    }
                })
                .0
        }
        let mut l = *weights.iter().max().unwrap();
        let mut r = weights.iter().sum::<i32>();
        while l < r {
            let m = (l + r) / 2;
            if check(&weights, m) <= days {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            15,
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3));
    }
}
