pub struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut i_min = -1;
        let mut i_max = -1;

        for (i, &n) in nums.iter().enumerate() {
            if n < min {
                min = n;
                i_min = i as i32;
            }

            if n > max {
                max = n;
                i_max = i as i32;
            }
        }

        let n = nums.len() as i32;

        if i_max == i_min {
            (i_min + 1).min(n - i_min)
        } else {
            if i_min > i_max {
                std::mem::swap(&mut i_min, &mut i_max);
            }
            let c1 = i_min + 1 + (n - i_max);
            let c2 = i_max + 1;
            let c3 = n - i_min;
            c1.min(c2.min(c3))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_deletions(vec![-11000]));
    }
}
