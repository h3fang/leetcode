pub struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let (mut prefix, mut suffix) = (vec![0; n], vec![0; n]);
        let mut s = Vec::with_capacity(max_heights.len());
        let mut result = 0;
        for (i, &h) in max_heights.iter().enumerate() {
            while !s.is_empty() && max_heights[*s.last().unwrap()] > h {
                s.pop();
            }
            if let Some(&j) = s.last() {
                prefix[i] = prefix[j] + (i - j) as i64 * h as i64;
            } else {
                prefix[i] = (i + 1) as i64 * h as i64;
            }
            s.push(i);
        }
        s.clear();
        for (i, &h) in max_heights.iter().enumerate().rev() {
            while !s.is_empty() && max_heights[*s.last().unwrap()] > h {
                s.pop();
            }
            if let Some(&j) = s.last() {
                suffix[i] = suffix[j] + (j - i) as i64 * h as i64;
            } else {
                suffix[i] = (n - i) as i64 * h as i64;
            }
            s.push(i);
            result = result.max(prefix[i] + suffix[i] - h as i64);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(22, Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(18, Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]));
    }
}
