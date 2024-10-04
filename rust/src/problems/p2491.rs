pub struct Solution;

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let n = skill.len();
        skill.sort_unstable();
        let mut result = (skill[0] * skill[n - 1]) as i64;
        let sum = skill[0] + skill[n - 1];
        for i in 1..n / 2 {
            if skill[i] + skill[n - i - 1] != sum {
                return -1;
            }
            result += (skill[i] * skill[n - i - 1]) as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(22, Solution::divide_players(vec![3, 2, 5, 1, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::divide_players(vec![3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::divide_players(vec![1, 1, 2, 3]));
    }
}
