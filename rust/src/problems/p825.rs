pub struct Solution;

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut count = [0; 121];
        for age in ages {
            count[age as usize] += 1;
        }
        let mut prefix = [0; 121];
        for age in 1..=120 {
            prefix[age] = prefix[age - 1] + count[age];
        }
        let mut result = 0;
        for age in 15..=120 {
            if count[age] > 0 {
                let lb = age / 2 + 7 + 1;
                result += (prefix[age] - prefix[lb - 1] - 1) * count[age];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::num_friend_requests(vec![20, 30, 100, 110, 120])
        );
    }
}
