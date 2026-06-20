pub struct Solution;

impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.extend([vec![1, 0], vec![n, i32::MAX]]);
        restrictions.sort_unstable_by_key(|r| r[0]);

        let mut h = vec![0; restrictions.len()];

        for (i, r) in restrictions.iter().enumerate().skip(1) {
            h[i] = (h[i - 1] + r[0] - restrictions[i - 1][0]).min(r[1]);
        }

        for (i, r) in restrictions.iter().enumerate().rev().skip(1) {
            h[i] = h[i].min(h[i + 1] + restrictions[i + 1][0] - r[0]);
        }

        let mut ans = 0;
        for (r, h) in restrictions.windows(2).zip(h.windows(2)) {
            ans = ans.max(r[1][0] - r[0][0] + h[0] + h[1]);
        }
        ans / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let restrictions = [[2, 1], [4, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::max_building(5, restrictions));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::max_building(6, vec![]));
    }

    #[test]
    fn case3() {
        let restrictions = [[5, 3], [2, 5], [7, 4], [10, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::max_building(10, restrictions));
    }
}
