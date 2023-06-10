pub struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        fn f(s: &[u8]) -> i16 {
            let mut c = 0;
            let mut min = b'z';
            for &b in s {
                match b.cmp(&min) {
                    std::cmp::Ordering::Less => {
                        min = b;
                        c = 1;
                    }
                    std::cmp::Ordering::Equal => c += 1,
                    std::cmp::Ordering::Greater => {}
                }
            }
            c
        }
        let mut ws = words
            .into_iter()
            .map(|w| f(w.as_bytes()))
            .collect::<Vec<_>>();
        ws.sort_unstable_by_key(|&w| -w);
        queries
            .iter()
            .map(|q| {
                let c = f(q.as_bytes());
                ws.partition_point(|&w| c < w) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = ["cbd"].iter().map(|w| w.to_string()).collect();
        let words = ["zaaaz"].iter().map(|w| w.to_string()).collect();
        assert_eq!(vec![1], Solution::num_smaller_by_frequency(queries, words));
    }

    #[test]
    fn case2() {
        let queries = ["bbb", "cc"].iter().map(|w| w.to_string()).collect();
        let words = ["a", "aa", "aaa", "aaaa"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(
            vec![1, 2],
            Solution::num_smaller_by_frequency(queries, words)
        );
    }
}
