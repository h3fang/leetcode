const SIZE: i32 = 7001;

pub struct MyHashMap {
    arr: [Vec<(i32, i32)>; SIZE as usize],
}

impl MyHashMap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        const VAL: Vec<(i32, i32)> = Vec::new();
        Self {
            arr: [VAL; SIZE as usize],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let idx = Self::hash(key);
        if let Some(e) = self.arr[idx].iter_mut().find(|e| e.0 == key) {
            e.1 = value;
        } else {
            self.arr[idx].push((key, value));
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let idx = Self::hash(key);
        self.arr[idx]
            .iter()
            .find(|e| e.0 == key)
            .map_or(-1, |e| e.1)
    }

    pub fn remove(&mut self, key: i32) {
        let idx = Self::hash(key);
        if let Some(e) = self.arr[idx].iter().position(|e| e.0 == key) {
            self.arr[idx].swap_remove(e);
        }
    }

    fn hash(key: i32) -> usize {
        (key % SIZE) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut my_hashmap = MyHashMap::new();
        my_hashmap.put(1, 1); // myHashMap 现在为 [[1,1]]
        my_hashmap.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]
        assert_eq!(1, my_hashmap.get(1)); // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
        assert_eq!(-1, my_hashmap.get(3)); // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
        my_hashmap.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
        assert_eq!(1, my_hashmap.get(2)); // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
        my_hashmap.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
        assert_eq!(-1, my_hashmap.get(2)); // 返回 -1（未找到），myHashMap 现在为 [[1,1]]
    }
}
