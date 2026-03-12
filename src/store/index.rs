use std::collections::HashMap;

/// Index maps key -> (segment_id, offset, value_len)
pub type IndexMap = HashMap<String, (usize, u64, u64)>;

#[derive(Default, Debug)]
pub struct Index {
    pub kv_map: IndexMap,
}

impl Index {
    pub fn new() -> Self {
        Index { 
            kv_map: HashMap::new() 
        }
    }

    pub fn insert(&mut self, key: String, segment_id: usize, offset: u64, value_len: u64) {
        self.kv_map.insert(key, (segment_id, offset, value_len));
    }

    pub fn remove(&mut self, key: &str) {
        self.kv_map.remove(key);
    }

    pub fn get(&self, key: &str) -> Option<&(usize, u64, u64)> {
        self.kv_map.get(key)
    }
}