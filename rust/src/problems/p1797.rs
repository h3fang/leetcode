pub struct Solution;

use std::collections::HashMap;

pub struct AuthenticationManager {
    map: HashMap<String, i32>,
    ttl: i32,
}

impl AuthenticationManager {
    pub fn new(time_to_live: i32) -> Self {
        Self {
            map: Default::default(),
            ttl: time_to_live,
        }
    }

    pub fn generate(&mut self, token_id: String, current_time: i32) {
        *self.map.entry(token_id).or_default() = current_time + self.ttl;
    }

    pub fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(t) = self.map.get_mut(&token_id) {
            if *t > current_time {
                *t = self.ttl + current_time;
            }
        }
    }

    pub fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.map.values().filter(|&&t| t > current_time).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut am = AuthenticationManager::new(5);
        am.renew("aaa".to_string(), 1);
        am.generate("aaa".to_string(), 2);
        assert_eq!(1, am.count_unexpired_tokens(6));
        am.generate("bbb".to_string(), 7);
        am.renew("aaa".to_string(), 8);
        am.renew("bbb".to_string(), 10);
        assert_eq!(0, am.count_unexpired_tokens(15));
    }
}
