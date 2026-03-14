use std::collections::HashMap;

/// Index maps kv [(key) -> (seg_id, offset, len)]
pub type IndexMap = HashMap<String, (usize, u64, u64)>;

pub struct Index {
    pub kv_map: IndexMap,
}

impl Index {
    pub fn new() -> Self {
        Self {
            kv_map: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, seg_id: usize, offset: u64, len: u64) {
        self.kv_map.insert(key, (seg_id, offset, len));
    }

    pub fn get(&self, key: &str) -> Option<&(usize, u64, u64)> {
        self.kv_map.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.kv_map.remove(key);
    }

    pub fn len(&self) -> usize {
        self.kv_map.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.kv_map.is_empty()
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}