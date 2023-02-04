pub struct Solution;

impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        let mut max = 0;
        coins.sort_unstable();
        for c in coins {
            if max + 1 < c {
                break;
            }
            max += c;
        }
        max + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::get_maximum_consecutive(vec![1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::get_maximum_consecutive(vec![1, 1, 1, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(20, Solution::get_maximum_consecutive(vec![1, 4, 10, 3, 1]));
    }
}
