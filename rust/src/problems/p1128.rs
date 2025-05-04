pub struct Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut freq = [0; 100];
        for d in dominoes {
            let a = d[0].min(d[1]);
            let b = d[0].max(d[1]);
            let idx = a * 10 + b;
            freq[idx as usize] += 1;
        }
        freq.into_iter().map(|f| f * (f - 1) / 2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let dominoes = [[1, 2], [2, 1], [3, 4], [5, 6]]
            .iter()
            .map(|d| d.to_vec())
            .collect();
        assert_eq!(1, Solution::num_equiv_domino_pairs(dominoes));
    }

    #[test]
    fn case2() {
        let dominoes = [[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]]
            .iter()
            .map(|d| d.to_vec())
            .collect();
        assert_eq!(3, Solution::num_equiv_domino_pairs(dominoes));
    }
}
