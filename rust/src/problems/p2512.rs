pub struct Solution;

use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let k = k as usize;
        let pos = positive_feedback
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<_>>();
        let neg = negative_feedback
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<_>>();
        let mut q = BinaryHeap::new();
        for (r, id) in report.into_iter().zip(student_id) {
            let score = r
                .split_ascii_whitespace()
                .map(|w| {
                    if pos.contains(w) {
                        3
                    } else if neg.contains(w) {
                        -1
                    } else {
                        0
                    }
                })
                .sum::<i32>();
            q.push((-score, id));
            if q.len() > k {
                q.pop();
            }
        }
        let mut result = Vec::with_capacity(k);
        while let Some((_, id)) = q.pop() {
            result.push(id);
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let positive_feedback = ["smart", "brilliant", "studious"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let negative_feedback = ["not"].iter().map(|w| w.to_string()).collect();
        let report = ["this student is studious", "the student is smart"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let student_id = vec![1, 2];
        let k = 2;
        assert_eq!(
            vec![1, 2],
            Solution::top_students(positive_feedback, negative_feedback, report, student_id, k)
        );
    }

    #[test]
    fn case2() {
        let positive_feedback = ["smart", "brilliant", "studious"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let negative_feedback = ["not"].iter().map(|w| w.to_string()).collect();
        let report = ["this student is not studious", "the student is smart"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let student_id = vec![1, 2];
        let k = 2;
        assert_eq!(
            vec![2, 1],
            Solution::top_students(positive_feedback, negative_feedback, report, student_id, k)
        );
    }
}
