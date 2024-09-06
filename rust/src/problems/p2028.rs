pub struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let sum: i32 = rolls.iter().sum();
        let r = mean * (n + rolls.len() as i32) - sum;
        if r < n || r > 6 * n {
            vec![]
        } else {
            let e = r / n;
            let remaining = r - (r / n) * n;
            let mut result = vec![e; (n - remaining) as usize];
            result.extend(vec![e + 1; remaining as usize]);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rolls = vec![3, 2, 4, 3];
        let mean = 4;
        let n = 2;
        let expected = [6, 6];
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(expected.len(), result.len());
        assert_eq!(expected.iter().sum::<i32>(), result.iter().sum::<i32>());
    }

    #[test]
    fn case2() {
        let rolls = vec![1, 5, 6];
        let mean = 3;
        let n = 4;
        let expected = [2, 3, 2, 2];
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(expected.len(), result.len());
        assert_eq!(expected.iter().sum::<i32>(), result.iter().sum::<i32>());
    }

    #[test]
    fn case3() {
        let rolls = vec![1, 2, 3, 4];
        let mean = 6;
        let n = 4;
        assert!(Solution::missing_rolls(rolls, mean, n).is_empty());
    }

    #[test]
    fn case4() {
        let rolls = vec![1];
        let mean = 3;
        let n = 1;
        let expected = [5];
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(expected.len(), result.len());
        assert_eq!(expected.iter().sum::<i32>(), result.iter().sum::<i32>());
    }
}
