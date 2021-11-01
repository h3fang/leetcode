pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut d = vec![0; nums.len() + 1];
        d[1] = nums[0];
        let mut len = 1;

        for &e in nums.iter().skip(1) {
            if e > d[len] {
                len += 1;
                d[len] = e;
            } else {
                let mut a = 1;
                let mut b = len;
                let mut p = 0;
                while a <= b {
                    let mid = a + (b - a) / 2;
                    match d[mid as usize].cmp(&e) {
                        std::cmp::Ordering::Less => {
                            p = mid;
                            a = mid + 1;
                        }
                        _ => b = mid - 1,
                    }
                }
                d[p + 1] = e;
            }
        }

        len as i32
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
