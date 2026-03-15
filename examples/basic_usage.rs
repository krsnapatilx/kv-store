use kv_store::KvStore;

fn main() -> std::io::Result<()> {
    println!("=== Basic KvStore Usage ===\n");

    // Create or open a store
    let mut store = KvStore::open("basic_usage")?;
    println!("✓ Store opened");

    // Set some key-value pairs
    store.set("user1:name", b"krishna")?;
    store.set("user1:email", b"krishna@gmail.com")?;
    store.set("user2:name", b"rukmini")?;
    store.set("user2:email", b"rukmini@gmail.com")?;
    println!("✓ Added 4 keys");

    // Get values
    if let Some(name) = store.get("user1:name")? {
        println!("✓ User 1 name: {}", String::from_utf8_lossy(&name));
    }

    // Update a value
    store.set("user1:email", b"krsna.new@example.com")?;
    println!("✓ Updated user1:email");

    // Delete a key
    store.delete("user2:email")?;
    println!("✓ Deleted user2:email");

    // List all keys
    let keys = store.list_keys();
    println!("\n✓ Current keys ({}):", keys.len());
    for key in keys {
        println!("  - {}", key);
    }

    // Show statistics
    let stats = store.stats();
    println!("\n✓ Store statistics:");
    println!("{}", stats);

    println!("\n✓ Example completed successfully!");

    Ok(())
}
