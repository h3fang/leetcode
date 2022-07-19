pub struct Solution;

impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(queries.len());
        let n = nums.len();
        let m = nums[0].as_bytes().len();
        for q in queries {
            let k = q[0] as usize - 1;
            let trim = q[1] as usize;
            let mut a = (0..n).collect::<Vec<_>>();
            a.sort_by_key(|&i| &nums[i][m - trim..]);
            result.push(a[k] as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = ["102", "473", "251", "814"];
        let queries = [[1, 1], [2, 3], [4, 2], [1, 2]];
        let nums = nums.iter().map(|n| n.to_string()).collect();
        let queries = queries.iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![2, 2, 1, 0],
            Solution::smallest_trimmed_numbers(nums, queries)
        );
    }

    #[test]
    fn case2() {
        let nums = ["24", "37", "96", "04"];
        let queries = [[2, 1], [2, 2]];
        let nums = nums.iter().map(|n| n.to_string()).collect();
        let queries = queries.iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![3, 0],
            Solution::smallest_trimmed_numbers(nums, queries)
        );
    }
}
