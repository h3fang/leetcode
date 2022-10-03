pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let c = colors.as_bytes();
        let mut last = b' ';
        let mut max = 0;
        let mut sum = 0;
        let mut reuslt = 0;
        for (i, (&color, &time)) in c.iter().zip(&needed_time).enumerate() {
            if color != last {
                reuslt += sum - max;
                last = c[i];
                max = time;
                sum = max;
            } else {
                max = max.max(time);
                sum += time;
            }
        }
        reuslt + sum - max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_cost("abc".to_string(), vec![1, 2, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1])
        );
    }
}
