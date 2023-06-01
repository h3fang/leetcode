pub struct Solution;

impl Solution {
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        fn check(price: &[i32], k: i32, mid: i32) -> bool {
            let mut prev = i32::MIN / 2;
            let mut c = 0;
            for &p in price {
                if p - prev >= mid {
                    c += 1;
                    prev = p;
                }
            }
            c >= k
        }
        price.sort_unstable();
        let mut left = 0;
        let mut right = *price.last().unwrap() - price[0];
        while left < right {
            let mid = left + (right - left + 1) / 2;
            if check(&price, k, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::maximum_tastiness(vec![1, 3, 1], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::maximum_tastiness(vec![7, 7, 7, 7], 2));
    }
}
