use kv_store::KvStore;

use std::fs::{create_dir_all, remove_dir_all};
use std::io::Result;
use std::path::Path;

fn setup_test_dir(path: &str) -> Result<()> {
    let p = Path::new(path);
    remove_dir_all(p)?;
    create_dir_all(p)
}

#[test]
fn can_set_and_get_value() -> std::io::Result<()> {
    let test_dir = "tests_data/can_set_and_get_value";

    setup_test_dir(test_dir)?;

    let mut store = KvStore::open(test_dir)?;

    store.set("foo", b"bar")?;

    assert_eq!(store.get("foo")?, Some(b"bar".to_vec()));

    Ok(())
}

#[test]
fn can_delete_value() -> std::io::Result<()> {
    let test_dir = "tests_data/can_delete_value";
    setup_test_dir(test_dir)?;

    let mut store = KvStore::open(test_dir)?;

    store.set("foo", b"bar")?;
    store.delete("foo")?;

    assert_eq!(store.get("foo")?, None);

    Ok(())
}

#[test]
fn overwriting_value_updates_storage() -> std::io::Result<()> {
    let test_dir = "tests_data/overwrite_value";
    setup_test_dir(test_dir)?;

    let mut store = KvStore::open(test_dir)?;

    store.set("key", b"value1")?;
    store.set("key", b"value2")?;

    assert_eq!(store.get("foo")?, Some(b"value2".to_vec()));

    Ok(())
}

#[test]
fn missing_key_returns_none() -> std::io::Result<()> {
    let test_dir = "tests_data/missing_key";
    setup_test_dir(test_dir)?;

    let mut store = KvStore::open(test_dir)?;

    assert_eq!(store.get("does_not_exist")?, None);

    Ok(())
}

#[test]
fn delete_nonexistent_key_is_safe() -> std::io::Result<()> {
    let test_dir = "tests_data/delete_nonexistent";
    setup_test_dir(test_dir)?;

    let mut store = KvStore::open(test_dir)?;

    store.delete("nope")?;
    assert_eq!(store.get("nope")?, None);

    Ok(())
}

#[test]
fn persistence_after_reopen() -> std::io::Result<()> {
    let test_dir = "tests_data/persistence";
    setup_test_dir(test_dir)?;

    {
        let mut store = KvStore::open(test_dir)?;
        store.set("persistent", b"value")?;
    }
    
    let mut store = KvStore::open(test_dir)?;
    assert_eq!(store.get("persistent")?, Some(b"value".to_vec()));

    Ok(())
}

#[test]
fn compaction_preserves_data() -> std::io::Result<()> {
    let test_dir = "tests_data/compaction";
    setup_test_dir(test_dir)?;

    let mut store = KvStore::open(test_dir)?;

    store.set("key1", b"value1")?;
    store.set("key2", b"value2")?;
    store.set("key3", b"value3")?;

    store.set("key1", b"updated1")?;
    store.delete("key2")?;

    store.compact()?;

    assert_eq!(store.get("key1")?, Some(b"updated1".to_vec()));
    assert_eq!(store.get("key1")?, None);
    assert_eq!(store.get("key1")?, Some(b"updated1".to_vec()));

    Ok(())
}