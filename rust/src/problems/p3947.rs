pub struct Solution;

impl Solution {
    pub fn maximum_sale_items(mut items: Vec<Vec<i32>>, mut budget: i32) -> i32 {
        items.sort_unstable_by_key(|e| e[1]);
        let max = items.iter().map(|e| e[0]).max().unwrap() as usize;

        let mut freq = vec![0; max + 1];
        for e in &items {
            freq[e[0] as usize] += 1;
        }

        for i in 1..=max {
            if freq[i] == 0 {
                continue;
            }

            for j in (i * 2..=max).step_by(i) {
                freq[i] += freq[j];
            }
        }

        let min = items[0][1];
        let mut ans = 0;

        for e in items {
            let (factor, price) = (e[0] as usize, e[1]);
            if price > min * 2 {
                break;
            }

            let n = (freq[factor] - 1).min(budget / price);
            ans += 2 * n;
            budget -= n * price;
        }

        ans + budget / min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let items: Vec<Vec<i32>> = vec![vec![1, 6], vec![2, 4], vec![3, 5]];
        let budget: i32 = 19;
        assert_eq!(5, Solution::maximum_sale_items(items, budget));
    }

    #[test]
    fn case2() {
        let items: Vec<Vec<i32>> = vec![
            vec![2, 8],
            vec![1, 10],
            vec![6, 6],
            vec![4, 12],
            vec![5, 20],
            vec![5, 17],
        ];
        let budget: i32 = 35;
        assert_eq!(7, Solution::maximum_sale_items(items, budget));
    }
}
