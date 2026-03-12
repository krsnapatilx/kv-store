use crate::store::engine::KvStore;
use crate::store::segment::Segment;

use std::fs;
use std::io::Result;

/// Naive compaction: write a new segment (next id) with latest values for all keys,
/// then remove old segments and load the new one.
/// 
/// This is simple and blocking - fine for a learning project / demo.
pub fn compact_segments(store: &mut KvStore) -> Result<()> {
    // compute next id
    let next_id = store
        .segments
        .keys()
        .copied()
        .max()
        .map_or(0, |m| m + 1);

    let dir = store.dir.clone();
    let mut new_seq = Segment::open(&dir, next_id)?;

    let keys: Vec<String> = store.index.kv_map.keys().cloned().collect();

    for key in keys {
        if let Some((seg_id, offset, _value_len)) = store.index.get(key.as_str()) {
            if let Some(seg) = store.segments.get_mut(seg_id) {
                if let Ok(Some(value)) = seg.read_value_at(*offset) {
                    let off = new_seq.append(key.as_bytes(), &value)?;

                    store
                        .index
                        .insert(key.to_string(), next_id, off, value.len() as u64);
                }
            }
        }
    }
    // delete old segments files (conservative: remove only segment-<id>.dat for ids < next_id)
    let ids_to_remove: Vec<usize> = store
        .segments
        .keys()
        .copied()
        .filter(|&id| id < next_id)
        .collect();

    for id in ids_to_remove {
        let fname = format!("segment-{}.dat", id);
        let path = dir.join(fname);
        
        fs::remove_file(path)?;
        store.segments.remove(&id);
    }

    store.segments.insert(next_id, new_seq);
    store.active_id = next_id;

    Ok(())

}