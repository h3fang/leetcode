pub struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let (mut max_factor, mut min_price) = (0, i32::MAX);
        for item in &items {
            max_factor = max_factor.max(item[0]);
            min_price = min_price.min(item[1]);
        }

        let mut cnt = vec![0; max_factor as usize + 1];
        for item in &items {
            cnt[item[0] as usize] += 1;
        }

        let mut multi = vec![0; max_factor as usize + 1];
        let mut f = vec![0; budget as usize + 1];
        let mut sum = 0;

        for item in items {
            let c = &mut multi[item[0] as usize];
            if *c == 0 {
                for j in (item[0]..=max_factor).step_by(item[0] as usize) {
                    *c += cnt[j as usize];
                }
            }

            sum = (sum + item[1]).min(budget);
            for j in (item[1]..=sum).rev() {
                f[j as usize] = f[j as usize].max(f[(j - item[1]) as usize] + *c);
            }
        }

        let mut ans = 0;
        for (i, f) in f.iter().enumerate().take(budget as usize + 1) {
            ans = ans.max(f + (budget - i as i32) / min_price);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let items: Vec<Vec<i32>> = vec![vec![6, 2], vec![2, 6], vec![3, 4]];
        let budget: i32 = 9;
        assert_eq!(4, Solution::maximum_sale_items(items, budget));
    }

    #[test]
    fn case2() {
        let items: Vec<Vec<i32>> =
            vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]];
        let budget: i32 = 8;
        assert_eq!(10, Solution::maximum_sale_items(items, budget));
    }
}
