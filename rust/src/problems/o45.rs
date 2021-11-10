pub struct Solution;

impl Solution {
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| (a.to_string() + b).cmp(&(b.to_string() + a)));
        nums.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casd1() {
        assert_eq!("102".to_string(), Solution::min_number(vec![10, 2]));
    }

    #[test]
    fn casd2() {
        assert_eq!(
            "3033459".to_string(),
            Solution::min_number(vec![3, 30, 34, 5, 9])
        );
    }

    #[test]
    fn casd3() {
        assert_eq!("12112".to_string(), Solution::min_number(vec![12, 121]));
    }
}
