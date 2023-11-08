use std::collections::HashMap;

fn generate_map_entry(adventure: u64, adventurer: u64) -> u128 {
    let mut record = adventure as u128;
    record = record << 64;
    record = record | adventurer as u128;
    record
}

fn check_for_entry(entry: u128, map: &HashMap<u128, ()>) -> Result<(), ()> {
    match map.get(&entry) {
        Some(_) => Ok(()),
        None => Err(())
    }
}

fn register_entry(entry: u128, map: &mut HashMap<u128, ()>) {
    map.insert(entry, ());
}

pub fn register_data(adventure: u64, adventurer: u64, map: &mut HashMap<u128, ()>) -> Result<(), ()> {
    let entry = generate_map_entry(adventure, adventurer);
    match check_for_entry(entry, map) {
        Ok(()) => {
            Err(())
        },
        Err(()) => {
            register_entry(entry, map);
            Ok(())
        }
    }
}
