pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut s = Vec::with_capacity(temperatures.len());
        let mut result = vec![0; temperatures.len()];
        for (i, t) in temperatures.iter().enumerate() {
            while !s.is_empty() && &temperatures[*s.last().unwrap()] < t {
                let j = s.pop().unwrap();
                result[j] = (i - j) as i32;
            }
            s.push(i);
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
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 1, 1, 0],
            Solution::daily_temperatures(vec![30, 40, 50, 60])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![1, 1, 0],
            Solution::daily_temperatures(vec![30, 60, 90])
        );
    }
}
