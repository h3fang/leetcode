pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX / 2;
        let mut max = i32::MIN / 2;
        let mut ans = 0;
        for a in arrays {
            let m = a.len();
            ans = ans.max(a[m - 1] - min).max(max - a[0]);
            min = min.min(a[0]);
            max = max.max(a[m - 1]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let a = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
        assert_eq!(4, Solution::max_distance(a));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_distance(vec![vec![1], vec![1]]));
    }
}
