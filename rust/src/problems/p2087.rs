pub struct Solution;

impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let mut result = 0;

        for (i, costs) in [row_costs, col_costs].iter().enumerate() {
            let min = home_pos[i].min(start_pos[i]) as usize;
            let max = home_pos[i].max(start_pos[i]) as usize;
            for c in &costs[min..=max] {
                result += c;
            }
            result -= costs[start_pos[i] as usize];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let start_pos = vec![1, 0];
        let home_pos = vec![2, 3];
        let row_costs = vec![5, 4, 3];
        let col_costs = vec![8, 2, 6, 7];
        assert_eq!(
            18,
            Solution::min_cost(start_pos, home_pos, row_costs, col_costs)
        );
    }
}
