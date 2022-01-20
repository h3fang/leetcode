pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        fn is_valid(piles: &[i32], k: i32, h: i32) -> bool {
            let mut c = 0;
            for p in piles {
                c += (p - 1) / k + 1;
                if c > h {
                    return false;
                }
            }
            c <= h
        }
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            if is_valid(&piles, mid, h) {
                right = mid;
            } else {
                left = mid + 1;
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
        assert_eq!(4, Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    }

    #[test]
    fn case2() {
        assert_eq!(30, Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5));
    }
}
