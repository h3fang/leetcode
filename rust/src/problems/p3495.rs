pub struct Solution;

fn f(x: i32) -> i64 {
    let x = x as i64;
    let (mut ans, mut y, mut i) = (0, 1, 1);
    while y * 4 <= x {
        ans += 3 * y * i;
        y *= 4;
        i += 1;
    }
    if y < x {
        ans += (x - y + 1) * i;
    }
    ans
}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        queries
            .iter()
            .map(|q| {
                let max = (33 - q[1].leading_zeros() as i64) / 2;
                let sum = f(q[1]) - f(q[0] - 1);
                ((sum + 1) / 2).max(max)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[1, 2], [2, 4]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(3, Solution::min_operations(queries));
    }

    #[test]
    fn case2() {
        let queries = [[2, 6]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(4, Solution::min_operations(queries));
    }
}
