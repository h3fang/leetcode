pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut d = Vec::with_capacity(nums.len() + 1);
        d.push(i32::MIN);
        d.push(nums[0]);

        for &e in nums.iter().skip(1) {
            if e > *d.last().unwrap() {
                d.push(e);
            } else {
                let p = d.partition_point(|&x| x < e);
                d[p] = e;
            }
        }

        d.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::length_of_lis(vec![0]));
    }
}
