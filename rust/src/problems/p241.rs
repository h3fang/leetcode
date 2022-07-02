pub struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut nums = vec![];
        let mut ops = vec![];
        let mut start = 0;
        for (i, &b) in expression.as_bytes().iter().enumerate() {
            if !b.is_ascii_digit() {
                nums.push(expression[start..i].parse::<i32>().unwrap());
                ops.push(b);
                start = i + 1;
            }
        }
        nums.push(expression[start..].parse::<i32>().unwrap());
        let n = nums.len();
        let mut cache = vec![vec![Vec::new(); n]; n];

        fn dfs(i: usize, j: usize, nums: &[i32], ops: &[u8], cache: &mut [Vec<Vec<i32>>]) {
            if !cache[i][j].is_empty() {
                return;
            }
            if i == j {
                cache[i][j].push(nums[i]);
                return;
            }

            for k in i..j {
                dfs(i, k, nums, ops, cache);
                dfs(k + 1, j, nums, ops, cache);
                for a in cache[i][k].clone() {
                    for b in cache[k + 1][j].clone() {
                        cache[i][j].push(match ops[k] {
                            b'+' => a + b,
                            b'-' => a - b,
                            _ => a * b,
                        });
                    }
                }
            }
        }

        if n == 1 {
            return nums;
        }

        let mut result = vec![];
        dfs(0, n - 1, &nums, &ops, &mut cache);
        for &a in &cache[0][n - 1] {
            result.push(a);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::diff_ways_to_compute("2-1-1".into());
        result.sort_unstable();
        let expected = vec![0, 2];
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::diff_ways_to_compute("2*3-4*5".into());
        result.sort_unstable();
        let expected = vec![-34, -14, -10, -10, 10];
        assert_eq!(expected, result);
    }
}
