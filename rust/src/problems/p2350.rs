pub struct Solution;

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut seg = vec![0; k as usize + 1];
        let mut result = 1;
        let mut count = 0;
        for n in rolls {
            if seg[n as usize] < result {
                seg[n as usize] = result;
                count += 1;
                if count == k {
                    result += 1;
                    count = 0;
                }
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
            Solution::shortest_sequence(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::shortest_sequence(vec![1, 1, 2, 2], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(
            1,
            Solution::shortest_sequence(vec![1, 1, 3, 2, 2, 2, 3, 3], 4)
        );
    }
}
