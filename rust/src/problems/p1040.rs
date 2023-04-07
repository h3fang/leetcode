pub struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort_unstable();
        let n = stones.len();
        if stones[0] + n as i32 - 1 == stones[n - 1] {
            return vec![0, 0];
        }
        let max =
            (stones[n - 1] - stones[1] + 1).max(stones[n - 2] - stones[0] + 1) - (n as i32 - 1);
        let mut min = n as i32;
        let mut j = 0;
        for i in 0..n {
            if j + 1 >= n {
                break;
            }
            while j + 1 < n && stones[j + 1] - stones[i] < n as i32 {
                j += 1;
            }
            if j - i + 1 == n - 1 && stones[j] - stones[i] + 1 == n as i32 - 1 {
                min = min.min(2);
            } else {
                min = min.min((n - (j - i + 1)) as i32);
            }
        }
        vec![min, max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 2], Solution::num_moves_stones_ii(vec![7, 4, 9]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![2, 3],
            Solution::num_moves_stones_ii(vec![6, 5, 4, 3, 10])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![0, 0],
            Solution::num_moves_stones_ii(vec![100, 101, 104, 102, 103])
        );
    }
}
