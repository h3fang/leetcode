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
        ans += (x - y) * i;
    }
    ans
}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        queries
            .iter()
            .map(|q| {
                let a = f(q[1]);
                let max = a - f(q[1] - 1);
                let sum = a - f(q[0] - 1);
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
