pub struct Solution;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = [false; 101];
        for x in nums {
            set[x as usize] = true;
        }
        for x in (k..).step_by(k as usize) {
            if x > 100 || !set[x as usize] {
                return x;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::missing_multiple(vec![8, 2, 3, 4, 6], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::missing_multiple(vec![1, 4, 7, 10, 15], 5));
    }
}
