pub struct Solution;

impl Solution {
    pub fn min_cost(
        mut start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let mut result = 0;
        let dh = (home_pos[0] - start_pos[0]).signum();
        let dw = (home_pos[1] - start_pos[1]).signum();

        while start_pos != home_pos {
            if start_pos[0] != home_pos[0] {
                start_pos[0] += dh;
                result += row_costs[start_pos[0] as usize];
            }
            if start_pos[1] != home_pos[1] {
                start_pos[1] += dw;
                result += col_costs[start_pos[1] as usize];
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
