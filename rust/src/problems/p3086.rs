pub struct Solution;

impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let mut ones = Vec::with_capacity(nums.len());
        let (mut c, mut curr) = (0, 0);
        for (i, &x) in nums.iter().enumerate() {
            if x == 0 {
                c = c.max(curr);
                curr = 0;
            } else {
                curr += 1;
                ones.push(i);
            }
        }
        c = c.max(curr).min(3).min(k);
        if max_changes >= k - c {
            return (0.max(c - 1) + 2 * (k - c)) as i64;
        }
        let mut presum = vec![0; ones.len() + 1];
        for (i, x) in ones.iter().enumerate() {
            presum[i + 1] = presum[i] + x;
        }
        let mut result = i64::MAX;
        let size = (k - max_changes) as usize;
        for r in size..=ones.len() {
            let l = r - size;
            let i = l + size / 2;
            let v = ones[i];
            let s1 = v * (i - l) - (presum[i] - presum[l]);
            let s2 = presum[r] - presum[i] - v * (r - i);
            result = result.min((s1 + s2) as i64);
        }
        result + max_changes as i64 * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::minimum_moves(vec![0, 0, 0, 0], 2, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::minimum_moves(vec![1, 0, 1, 0, 1], 3, 0));
    }
}
