pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.iter().filter(|d| &d[11..13] > "60").count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let details = ["7868190130M7522", "5303914400F9211", "9273338290F4010"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(2, Solution::count_seniors(details));
    }

    #[test]
    fn case2() {
        let details = ["1313579440F2036", "2921522980M5644"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(0, Solution::count_seniors(details));
    }
}
