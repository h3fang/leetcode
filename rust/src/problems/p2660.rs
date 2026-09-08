pub struct Solution;

fn score(player: &[i32]) -> i32 {
    player
        .iter()
        .enumerate()
        .map(|(i, &e)| {
            if (i > 0 && player[i - 1] == 10) || (i > 1 && player[i - 2] == 10) {
                2 * e
            } else {
                e
            }
        })
        .sum::<i32>()
}

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let p1 = score(&player1);
        let p2 = score(&player2);
        match p1.cmp(&p2) {
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::is_winner(vec![2, 3], vec![4, 1]));
    }
}
