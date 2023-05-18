use std::collections::BTreeMap;
use talk_lib::*;

/// Example usage of a metadata store implementation
/// 
fn main() -> Result<(), Error> {
    // Create an example store info
    let store_info = StoreInfo::new("testdb", "test");

    // Create an example memory store
    let mut memory_store = MemoryStore::<Descriptor>::from(store_info);

    // Create a new item
    let created = {
        let desc = Descriptor::new(
            "test-digest",
            "test-media-type", 
            0,
             BTreeMap::new()
            );
        memory_store.create(&desc)?.clone()
    };

    // Test reading the item
    let test = memory_store.read(&created)?;
    println!("read: {:#?}", test);

    // Test replacing the item
    let mut replacement = created.clone();
    replacement.set_digest("better-test-digest");
    let replaced = memory_store.replace(&replacement)?.clone();
    let test = memory_store.read(&created)?;
    println!("read replacement: {:#?}", test);
    println!("{} items", memory_store.iter_items().count());

    // Test deleting the item
    let deleted = memory_store.delete(&replaced)?;
    println!("deleted: {:#?}", deleted);
    println!("{} items", memory_store.iter_items().count());
    Ok(())
}
