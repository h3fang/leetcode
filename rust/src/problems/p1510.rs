pub struct Solution;

use std::sync::LazyLock;

static F: LazyLock<Vec<bool>> = LazyLock::new(|| {
    const N: usize = 10_0001;
    let mut f = vec![false; N];
    for i in 0..f.len() {
        if f[i] {
            continue;
        }
        for j in 1..N {
            if i + j * j >= N {
                break;
            }
            f[i + j * j] = true;
        }
    }
    f
});

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        F[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::winner_square_game(1));
    }

    #[test]
    fn case2() {
        assert!(!Solution::winner_square_game(2));
    }

    #[test]
    fn case3() {
        assert!(Solution::winner_square_game(4));
    }

    #[test]
    fn case4() {
        assert!(!Solution::winner_square_game(7));
    }

    #[test]
    fn case5() {
        assert!(Solution::winner_square_game(99));
    }
}
