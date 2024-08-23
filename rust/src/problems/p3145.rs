pub struct Solution;

fn pow(mut x: i64, mut n: i64, m: i64) -> i64 {
    let mut result = 1 % m;
    while n > 0 {
        if n % 2 == 1 {
            result = (result * x) % m;
        }
        x = (x * x) % m;
        n /= 2;
    }
    result
}

fn sum(mut x: i64) -> i64 {
    let (mut result, mut n, mut ones, mut s) = (0, 0i64, 0, 0);
    for i in (0..64 - (x + 1).leading_zeros() as i64).rev() {
        let c = (ones << i) + (i << i) / 2;
        if c <= x {
            x -= c;
            result += (s << i) + ((i * (i - 1) / 2) << i) / 2;
            s += i;
            ones += 1;
            n |= 1 << i;
        }
    }
    for _ in 0..x {
        result += n.trailing_zeros() as i64;
        n &= n - 1;
    }
    result
}

impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|q| {
                let l = sum(q[0]);
                let r = sum(q[1] + 1);
                pow(2, r - l, q[2]) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[1, 3, 7]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![4], Solution::find_products_of_elements(queries));
    }

    #[test]
    fn case2() {
        let queries = [[2, 5, 3], [7, 7, 4]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![2, 2], Solution::find_products_of_elements(queries));
    }
}
