pub struct Solution;

impl Solution {
    pub fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }
        let n = nums.len();
        let mut f = Vec::with_capacity(n);
        for &x in &nums {
            let mut v = vec![false; x as usize + 1];
            v[0] = true;
            f.push(v);
        }
        for (i, q) in queries.iter().enumerate() {
            let v = q[2];
            for i in q[0] as usize..=q[1] as usize {
                for j in (v..=nums[i]).rev() {
                    if f[i][(j - v) as usize] {
                        f[i][j as usize] = true;
                    }
                }
                if nums[i] > 0 && f[i][nums[i] as usize] {
                    nums[i] = 0;
                }
            }
            if nums.iter().all(|&x| x == 0) {
                return i as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 0, 2];
        let queries = [[0, 2, 1], [0, 2, 1], [1, 1, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(2, Solution::min_zero_array(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![4, 3, 2, 1];
        let queries = [[1, 3, 2], [0, 2, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(-1, Solution::min_zero_array(nums, queries));
    }

    #[test]
    fn case3() {
        let nums = vec![1, 2, 3, 2, 1];
        let queries = [[0, 1, 1], [1, 2, 1], [2, 3, 2], [3, 4, 1], [4, 4, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(4, Solution::min_zero_array(nums, queries));
    }

    #[test]
    fn case4() {
        let nums = vec![1, 2, 3, 2, 6];
        let queries = [
            [0, 1, 1],
            [0, 2, 1],
            [1, 4, 2],
            [4, 4, 4],
            [3, 4, 1],
            [4, 4, 5],
        ]
        .iter()
        .map(|q| q.to_vec())
        .collect();
        assert_eq!(4, Solution::min_zero_array(nums, queries));
    }
}
