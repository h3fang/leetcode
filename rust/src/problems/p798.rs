pub struct Solution;

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut diff = vec![0; n];
        for (i, &num) in nums.iter().enumerate() {
            let num = num as usize;
            if num <= i {
                diff[0] += 1;
                diff[(i - num + 1) % n] -= 1;
                diff[(i + 1) % n] += 1;
            } else {
                diff[(i + 1) % n] += 1;
                diff[(n - (num - i) + 1) % n] -= 1;
            }
        }
        let mut result = 0;
        let mut score = 0;
        let mut best = 0;
        for (i, d) in diff.iter().enumerate() {
            score += d;
            if score > best {
                best = score;
                result = i;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::best_rotation(vec![2, 3, 1, 4, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::best_rotation(vec![1, 3, 0, 2, 4]));
    }
}
