pub struct Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut count = [0; 1001];
        for n in target {
            count[n as usize] += 1;
        }
        for n in arr {
            count[n as usize] -= 1;
            if count[n as usize] < 0 {
                return false;
            }
        }
        count.iter().all(|c| *c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    }
}
