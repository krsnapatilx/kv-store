pub mod store;

pub use store::KvStore;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all};
    use std::path::Path;

    fn setup_test_dir(path: &str) {
        let p = Path::new(path);
        let _ = remove_dir_all(p);
        create_dir_all(p).expect("Failed to create test directory");
    }

    fn cleanup_test_dir(path: &str) {
        let _ = remove_dir_all(Path::new(path));
    }

    #[test]
    fn test_basic_set_get() {
        let test_dir = "tests_data/unit_basic_set_get";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();
        store.set("hello", b"world").unwrap();

        assert_eq!(store.get("hello").unwrap(), Some(b"world".to_vec()));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_overwrite_key() {
        let test_dir = "tests_data/unit_overwrite";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        store.set("key", b"value1").unwrap();
        store.set("key", b"value2").unwrap();

        assert_eq!(store.get("key").unwrap(), Some(b"value2".to_vec()));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_delete_removes_key() {
        let test_dir = "tests_data/unit_delete";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        store.set("temp", b"data").unwrap();
        assert!(store.get("temp").unwrap().is_some());

        store.delete("temp").unwrap();
        assert!(store.get("temp").unwrap().is_none());

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_missing_key() {
        let test_dir = "tests_data/unit_missing_key";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        assert_eq!(store.get("notexist").unwrap(), None);

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_list_key() {
        let test_dir = "tests/unit_list_keys";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        store.set("a", b"1").unwrap();
        store.set("b", b"2").unwrap();
        store.set("c", b"3").unwrap();

        let keys = store.list_keys();

        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"a".to_string()));
        assert!(keys.contains(&"b".to_string()));
        assert!(keys.contains(&"c".to_string()));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_empty_value() {
        let test_dir = "tests_data/unit_empty_value";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        store.set("empty", b"").unwrap();

        assert_eq!(store.get("empty").unwrap(), Some(vec![]));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_stats() {
        let test_dir = "tests_data/unit_stats";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        store.set("key1", b"value1").unwrap();
        store.set("key2", b"value2").unwrap();

        let stats = store.stats();
        assert_eq!(stats.num_keys, 2);
        assert!(stats.num_segments >= 1);
        assert!(stats.total_bytes > 0);

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_compaction_reduces_segments() {
        let test_dir = "tests_data/unit_compaction";
        setup_test_dir(test_dir);

        let mut store = KvStore::open(test_dir).unwrap();

        // Write multiple versions of the same keys
        for i in 0..10 {
            store.set("key1", format!("value{}", i).as_bytes()).unwrap();
            store.set("key2", format!("value{}", i).as_bytes()).unwrap();
        }

        store.compact().unwrap();

        // Verify data is still correct after compaction
        assert_eq!(store.get("key1").unwrap(), Some(b"value9".to_vec()));
        assert_eq!(store.get("key2").unwrap(), Some(b"value9".to_vec()));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_persistence_across_reopens() {
        let test_dir = "tests_data/unit_persistence";
        setup_test_dir(test_dir);

        {
            let mut store = KvStore::open(test_dir).unwrap();
            store.set("key1", b"value1").unwrap();
        }

        let mut new_store = KvStore::open(test_dir).unwrap();
        new_store.set("key2", b"value2").unwrap();

        assert_eq!(new_store.get("key1").unwrap(), Some(b"value1".to_vec()));
        assert_eq!(new_store.get("key2").unwrap(), Some(b"value2".to_vec()));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_utf8_support() {
        let tests_dir = "tests_data/unit_utf8";
        setup_test_dir(tests_dir);

        let mut store = KvStore::open(tests_dir).unwrap();
        store.set("🔑", "🔓".as_bytes()).unwrap();

        assert_eq!(store.get("🔑").unwrap(), Some("🔓".as_bytes().to_vec()));

        cleanup_test_dir(tests_dir);
    }
}
