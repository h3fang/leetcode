pub struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let (mut prefix, mut suffix) = (vec![0; n], vec![0; n]);
        let mut s = Vec::with_capacity(n);
        for (i, &a) in max_heights.iter().enumerate() {
            while !s.is_empty() && max_heights[*s.last().unwrap()] > a {
                s.pop();
            }
            let a = a as i64;
            prefix[i] = if s.is_empty() {
                a * (i as i64 + 1)
            } else {
                let j = *s.last().unwrap();
                prefix[j] + a * (i - j) as i64
            };
            s.push(i);
        }
        s.clear();
        let mut result = 0;
        for (i, &a) in max_heights.iter().enumerate().rev() {
            while !s.is_empty() && max_heights[*s.last().unwrap()] > a {
                s.pop();
            }
            let a = a as i64;
            suffix[i] = if s.is_empty() {
                a * (n - i) as i64
            } else {
                let j = *s.last().unwrap();
                suffix[j] + a * (j - i) as i64
            };
            result = result.max(prefix[i] + suffix[i] - a);
            s.push(i);
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
