pub struct Solution;

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut presum = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for (i, r) in matrix.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                presum[i + 1][j + 1] = presum[i + 1][j] ^ presum[i][j + 1] ^ presum[i][j] ^ c;
            }
        }
        let mut sums = presum.into_iter().flatten().collect::<Vec<_>>();
        sums.sort_unstable_by_key(|e| -e);
        sums[k as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[5, 2], [1, 6]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(7, Solution::kth_largest_value(matrix, 1));
    }

    #[test]
    fn case2() {
        let matrix = [[5, 2], [1, 6]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(5, Solution::kth_largest_value(matrix, 2));
    }

    #[test]
    fn case3() {
        let matrix = [[5, 2], [1, 6]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::kth_largest_value(matrix, 3));
    }

    #[test]
    fn case4() {
        let matrix = [[5, 2], [1, 6]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::kth_largest_value(matrix, 4));
    }
}
