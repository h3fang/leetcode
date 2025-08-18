pub struct Solution;

const EPS: f64 = 1e-9;

fn dfs(cards: Vec<f64>) -> bool {
    if cards.len() == 1 {
        return (cards[0] - 24.0).abs() < EPS;
    }

    for (i, x) in cards.iter().enumerate() {
        for (j, y) in cards.iter().enumerate().skip(i + 1) {
            let mut v = vec![x + y, x * y, x - y, y - x];
            if y.abs() > EPS {
                v.push(x / y);
            }
            if x.abs() > EPS {
                v.push(y / x);
            }
            for &r in &v {
                let mut new = cards.clone();
                new.remove(j);
                new[i] = r;
                if dfs(new) {
                    return true;
                }
            }
        }
    }
    false
}

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let cards = cards.into_iter().map(|c| c as f64).collect();
        dfs(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::judge_point24(vec![4, 1, 8, 7]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::judge_point24(vec![1, 2, 1, 2]));
    }
}
