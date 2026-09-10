use std::collections::HashMap;

#[derive(Default)]
pub struct FrequencyTracker {
    freq: HashMap<i32, i32>,
    f_freq: HashMap<i32, i32>,
}

impl FrequencyTracker {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, number: i32) {
        let e = self.freq.entry(number).or_default();
        *self.f_freq.entry(*e).or_default() -= 1;
        *e += 1;
        *self.f_freq.entry(*e).or_default() += 1;
    }

    pub fn delete_one(&mut self, number: i32) {
        if let Some(e) = self.freq.get_mut(&number) {
            if *e <= 0 {
                return;
            }
            *self.f_freq.entry(*e).or_default() -= 1;
            *e -= 1;
            *self.f_freq.entry(*e).or_default() += 1;
        }
    }

    pub fn has_frequency(&self, frequency: i32) -> bool {
        self.f_freq.get(&frequency).is_some_and(|&n| n > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ft = FrequencyTracker::new();
        ft.add(3);
        ft.add(3);
        assert!(ft.has_frequency(2));
    }

    #[test]
    fn case2() {
        let mut ft = FrequencyTracker::new();
        ft.add(1);
        ft.delete_one(1);
        assert!(!ft.has_frequency(1));
    }

    #[test]
    fn case3() {
        let mut ft = FrequencyTracker::new();
        assert!(!ft.has_frequency(2));
        ft.add(3);
        assert!(ft.has_frequency(1));
    }
}
