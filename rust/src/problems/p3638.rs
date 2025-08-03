pub struct Solution;

impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let n = weight.len();
        let (mut ans, mut i) = (0, 1);
        while i < n {
            if weight[i - 1] < weight[i] {
                ans += 1;
                i += 2;
            } else {
                i += 1;
            }
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
