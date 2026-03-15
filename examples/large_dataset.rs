use kv_store::KvStore;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    println!("=== Large Dataset Example ===\n");

    let mut store = KvStore::open("large_dataset")?;

    // Insert 10,000 keys
    println!("Inserting 25,000 keys...");
    let start = Instant::now();
    for i in 0..25_000 {
        let key = format!("user:{:05}:key", i);
        let value = format!("User value for key {}", i);
        store.set(key.as_str(), value.as_bytes())?;

        if (i + 1) % 2000 == 0 {
            println!("  {} keys inserted...", i + 1);
        }
    }

    let insert_duration = start.elapsed();
    println!(
        "✓ Insertion completed in {:.2}s",
        insert_duration.as_secs_f64()
    );

    // Read random keys
    println!("\nReading 2,500 random keys...");
    let start = Instant::now();
    for i in (0..25_000).step_by(10) {
        let key = format!("user:{:05}:key", i);
        let _ = store.get(key.as_str())?;
    }
    let read_duration = start.elapsed();
    println!("✓ Read completed in {:.2}s", read_duration.as_secs_f64());

    // Show statistics
    let stats = store.stats();
    println!("\n✓ Final statistics:");
    println!("{}", stats);

    println!("\nPerformance:");
    println!(
        "  Insert rate: {:.0} keys/sec",
        25_000.0 / insert_duration.as_secs_f64()
    );

    println!(
        "  Read rate: {:.0} keys/sec",
        2_500.0 / read_duration.as_secs_f64()
    );

    Ok(())
}
