pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut result = 0;
        let mut left = 0;
        let mut right = people.len() - 1;
        while left <= right {
            result += 1;
            if people[left] + people[right] <= limit {
                left += 1;
            }
            if right == 0 {
                break;
            }
            right -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let people = vec![1, 2];
        assert_eq!(1, Solution::num_rescue_boats(people, 3));
    }

    #[test]
    fn case2() {
        let people = vec![3, 2, 2, 1];
        assert_eq!(3, Solution::num_rescue_boats(people, 3));
    }

    #[test]
    fn case3() {
        let people = vec![3, 5, 3, 4];
        assert_eq!(4, Solution::num_rescue_boats(people, 5));
    }
}
