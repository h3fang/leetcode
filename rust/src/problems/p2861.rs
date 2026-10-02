pub struct Solution;

impl Solution {
    pub fn max_number_of_alloys(
        _n: i32,
        _k: i32,
        budget: i32,
        composition: Vec<Vec<i32>>,
        stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        let max_alloys_for_machine = |comp: &Vec<i32>| -> i32 {
            let (mut l, mut r) = (0, i32::MAX);
            while l + 1 < r {
                let m = l.midpoint(r);
                let c: i64 = comp
                    .iter()
                    .zip(&stock)
                    .zip(&cost)
                    .map(|((&comp, &stock), &cost)| {
                        (comp as i64 * m as i64 - stock as i64).max(0) * cost as i64
                    })
                    .sum();
                if c > budget as i64 {
                    r = m;
                } else {
                    l = m;
                }
            }
            l
        };
        composition
            .iter()
            .map(max_alloys_for_machine)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let composition = [[1, 1, 1], [1, 1, 10]].iter().map(|c| c.to_vec()).collect();
        assert_eq!(
            2,
            Solution::max_number_of_alloys(3, 2, 15, composition, vec![0, 0, 0], vec![1, 2, 3])
        );
    }

    #[test]
    fn case2() {
        let composition = [[1, 1, 1], [1, 1, 10]].iter().map(|c| c.to_vec()).collect();
        assert_eq!(
            5,
            Solution::max_number_of_alloys(3, 2, 15, composition, vec![0, 0, 100], vec![1, 2, 3])
        );
    }

    #[test]
    fn case3() {
        let composition = [[2, 1], [1, 2], [1, 1]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(
            2,
            Solution::max_number_of_alloys(2, 3, 10, composition, vec![1, 1], vec![5, 5])
        );
    }
}
