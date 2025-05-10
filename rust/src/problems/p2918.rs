pub struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (mut s1, mut z1, mut s2, mut z2) = (0, 0, 0, 0);
        for x in nums1 {
            if x == 0 {
                z1 += 1;
            } else {
                s1 += x as i64;
            }
        }
        for x in nums2 {
            if x == 0 {
                z2 += 1;
            } else {
                s2 += x as i64;
            }
        }
        if (s1 + z1 > s2 && z2 == 0) || (s2 + z2 > s1 && z1 == 0) {
            return -1;
        }
        (s1 + z1).max(s2 + z2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_sum(vec![2, 0, 2, 0], vec![1, 4]));
    }
}
