pub struct Solution;

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut players = vec![false; n as usize];
        players[0] = true;
        let mut curr = 0;
        for i in 1.. {
            let j = (curr + i * k) % n;
            if players[j as usize] {
                break;
            } else {
                players[j as usize] = true;
                curr = j;
            }
        }
        (1..=n).filter(|&i| !players[i as usize - 1]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![4, 5], Solution::circular_game_losers(5, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![2, 3, 4], Solution::circular_game_losers(4, 4));
    }
}
