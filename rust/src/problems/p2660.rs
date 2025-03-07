pub struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let scores = [player1, player2]
            .into_iter()
            .map(|p| {
                p.iter()
                    .enumerate()
                    .map(|(i, &e)| {
                        if (i > 0 && p[i - 1] == 10) || (i > 1 && p[i - 2] == 10) {
                            2 * e
                        } else {
                            e
                        }
                    })
                    .sum::<i32>()
            })
            .collect::<Vec<_>>();
        match scores[0].cmp(&scores[1]) {
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
