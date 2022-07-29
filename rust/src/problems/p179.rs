pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|n| n.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| {
            let a = a.as_bytes();
            let b = b.as_bytes();
            for (a, b) in a.iter().chain(b).zip(b.iter().chain(a)) {
                if a == b {
                    continue;
                } else {
                    return a.cmp(b);
                }
            }
            std::cmp::Ordering::Equal
        });
        let r = nums.into_iter().rev().collect::<String>();
        if r.chars().all(|c| c == '0') {
            "0".into()
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("210", Solution::largest_number(vec![10, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!("9534330", Solution::largest_number(vec![3, 30, 34, 5, 9]));
    }
}
