pub struct Solution;

impl Solution {
    pub fn maximum_beauty_nlogn_but_slow(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();
        for i in 1..items.len() {
            items[i][1] = items[i - 1][1].max(items[i][1]);
        }
        fn helper(q: i32, items: &[Vec<i32>]) -> i32 {
            let mut left = 0;
            let mut right = items.len() - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                match items[mid][0].cmp(&q) {
                    std::cmp::Ordering::Greater => {
                        if mid == 0 {
                            return 0;
                        }
                        right = mid - 1;
                    }
                    _ => {
                        left = mid + 1;
                    }
                }
            }
            items[right][1]
        }
        queries.into_iter().map(|q| helper(q, &items)).collect()
    }

    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|e| e[1]);
        fn helper(q: i32, items: &[Vec<i32>]) -> i32 {
            for item in items.iter().rev() {
                if q >= item[0] {
                    return item[1];
                }
            }
            0
        }
        queries.into_iter().map(|q| helper(q, &items)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let items = [[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]];
        let items = items.iter().map(|i| i.to_vec()).collect::<Vec<_>>();
        let queries = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            vec![2, 4, 5, 5, 6, 6],
            Solution::maximum_beauty_nlogn_but_slow(items.clone(), queries.clone())
        );
        assert_eq!(
            vec![2, 4, 5, 5, 6, 6],
            Solution::maximum_beauty(items, queries)
        );
    }

    #[test]
    fn case2() {
        let items = [[1, 2], [1, 2], [1, 3], [1, 4]];
        let items = items.iter().map(|i| i.to_vec()).collect::<Vec<_>>();
        let queries = vec![1];
        assert_eq!(
            vec![4],
            Solution::maximum_beauty_nlogn_but_slow(items.clone(), queries.clone())
        );
        assert_eq!(vec![4], Solution::maximum_beauty(items, queries));
    }

    #[test]
    fn case3() {
        let items = [
            [193, 732],
            [781, 962],
            [864, 954],
            [749, 627],
            [136, 746],
            [478, 548],
            [640, 908],
            [210, 799],
            [567, 715],
            [914, 388],
            [487, 853],
            [533, 554],
            [247, 919],
            [958, 150],
            [193, 523],
            [176, 656],
            [395, 469],
            [763, 821],
            [542, 946],
            [701, 676],
        ];
        let items = items.iter().map(|i| i.to_vec()).collect::<Vec<_>>();
        let queries = vec![
            885, 1445, 1580, 1309, 205, 1788, 1214, 1404, 572, 1170, 989, 265, 153, 151, 1479,
            1180, 875, 276, 1584,
        ];
        assert_eq!(
            vec![
                962, 962, 962, 962, 746, 962, 962, 962, 946, 962, 962, 919, 746, 746, 962, 962,
                962, 919, 962
            ],
            Solution::maximum_beauty_nlogn_but_slow(items.clone(), queries.clone())
        );
        assert_eq!(
            vec![
                962, 962, 962, 962, 746, 962, 962, 962, 946, 962, 962, 919, 746, 746, 962, 962,
                962, 919, 962
            ],
            Solution::maximum_beauty(items, queries)
        );
    }
}
