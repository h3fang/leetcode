pub struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut count = [0; 3];
        for s in stones {
            count[(s % 3) as usize] += 1;
        }
        if count[0] % 2 == 0 {
            count[1] >= 1 && count[2] >= 1
        } else {
            count[1] - count[2] > 2 || count[2] - count[1] > 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::stone_game_ix(vec![2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::stone_game_ix(vec![5, 1, 2, 4, 3]));
    }
}
