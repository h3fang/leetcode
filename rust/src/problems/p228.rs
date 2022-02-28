pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        let mut left = 0;
        let mut i = 0;
        while i < nums.len() {
            while i + 1 < nums.len() && nums[i] + 1 == nums[i + 1] {
                i += 1;
            }
            result.push(if left == i {
                nums[i].to_string()
            } else {
                format!("{}->{}", nums[left], nums[i])
            });
            i += 1;
            left = i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = ["0->2", "4->5", "7"];
        let expected = expected.iter().map(|r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]));
    }

    #[test]
    fn case2() {
        let expected = ["0", "2->4", "6", "8->9"];
        let expected = expected.iter().map(|r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );
    }
}
