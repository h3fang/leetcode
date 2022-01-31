pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|a| a.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let accounts = [[1, 2, 3], [3, 2, 1]];
        let accounts = accounts.iter().map(|a| a.to_vec()).collect();
        assert_eq!(6, Solution::maximum_wealth(accounts));
    }

    #[test]
    fn case2() {
        let accounts = [[1, 5], [7, 3], [3, 5]];
        let accounts = accounts.iter().map(|a| a.to_vec()).collect();
        assert_eq!(10, Solution::maximum_wealth(accounts));
    }
}
