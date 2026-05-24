pub struct Solution;

fn dfs(i: usize, left: &[usize], right: &[usize], cache: &mut [i32]) -> i32 {
    if i == left.len() {
        return 0;
    }

    if cache[i] != 0 {
        return cache[i];
    }

    let r = dfs(left[i], left, right, cache).max(dfs(right[i], left, right, cache)) + 1;
    cache[i] = r;
    r
}

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (arr.len(), d as usize);

        let mut s = Vec::with_capacity(n);
        let mut left = vec![0; n];
        for (i, &x) in arr.iter().enumerate() {
            while s.last().is_some_and(|&j| arr[j] <= x) {
                s.pop();
            }
            left[i] = if let Some(&j) = s.last()
                && i - j <= d
            {
                j
            } else {
                n
            };
            s.push(i);
        }

        s.clear();
        let mut right = vec![0; n];
        for (i, &x) in arr.iter().enumerate().rev() {
            while s.last().is_some_and(|&j| arr[j] <= x) {
                s.pop();
            }
            right[i] = if let Some(&j) = s.last()
                && j - i <= d
            {
                j
            } else {
                n
            };
            s.push(i);
        }

        let mut cache = vec![0; n];
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(dfs(i, &left, &right, &mut cache));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_jumps(vec![3, 3, 3, 3, 3], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(7, Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::max_jumps(vec![7, 1, 7, 1, 7, 1], 2));
    }

    #[test]
    fn case5() {
        assert_eq!(1, Solution::max_jumps(vec![66], 1));
    }
}
