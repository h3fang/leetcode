pub struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut g = vec![vec![]; nums.len() + 1];
        for (i, x) in nums.into_iter().enumerate() {
            g[x as usize].push(i as i32);
        }

        let mut ans = i32::MAX;
        for v in g {
            for w in v.windows(3) {
                let dist = w[0].abs_diff(w[2]);
                ans = ans.min(dist as i32);
            }
        }

        if ans == i32::MAX { -1 } else { 2 * ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::minimum_distance(vec![1, 2, 1, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]));
    }
}
