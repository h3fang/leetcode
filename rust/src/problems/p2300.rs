pub struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        spells
            .iter()
            .map(|&s| {
                let i = potions.partition_point(|&p| (s as i64 * p as i64) < success);
                (potions.len() - i) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        assert_eq!(
            vec![4, 0, 3],
            Solution::successful_pairs(spells, potions, success)
        );
    }
}
