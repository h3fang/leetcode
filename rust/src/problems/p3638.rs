pub struct Solution;

impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let n = weight.len();
        let (mut ans, mut i) = (0, 0);
        while i + 1 < n {
            let mut max = weight[i];
            let mut j = i + 1;
            while j < n && weight[j] >= max {
                max = weight[j];
                j += 1
            }
            if j < n {
                ans += 1;
            }
            i = j + 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_balanced_shipments(vec![2, 5, 1, 4, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_balanced_shipments(vec![4, 4]));
    }
}
