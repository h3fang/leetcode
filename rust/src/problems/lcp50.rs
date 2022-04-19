pub struct Solution;

impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for o in operations {
            let amount = gem[o[0] as usize] / 2;
            gem[o[0] as usize] -= amount;
            gem[o[1] as usize] += amount;
        }
        let min = *gem.iter().min().unwrap();
        let max = *gem.iter().max().unwrap();
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let gem = vec![3, 1, 2];
        let operations = [[0, 2], [2, 1], [2, 0]];
        let operations = operations.iter().map(|o| o.to_vec()).collect();
        assert_eq!(2, Solution::give_gem(gem, operations));
    }
}
