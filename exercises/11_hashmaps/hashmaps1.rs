// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.
/*!SECTION
 * 
 * use std::collections::HashMap;

let mut map = HashMap::new();
let map: HashMap<String, i32> = HashMap::from([
    ("key1".to_string(), 100),
    ("key2".to_string(), 200),
]);

Inserting/Updating

map.insert(key, value);           // Insert/overwrite, returns Option<V> (old value)
map.entry(key).or_insert(value);  // Insert only if key doesn't exist
map.entry(key).or_default();      // Insert default value if missing
map.entry(key).and_modify(|v| *v += 1).or_insert(0);  // Modify or insert

Accessing

map.get(&key);                    // Returns Option<&V>
map.get_mut(&key);                // Returns Option<&mut V>
map[&key];                        // Panics if key doesn't exist
map.contains_key(&key);           // Returns bool

Removing

map.remove(&key);                 // Returns Option<V>
map.remove_entry(&key);           // Returns Option<(K, V)>
map.clear();                      // Remove all entries

Iteration

for (key, value) in &map { }      // Iterate by reference
for (key, value) in &mut map { }  // Iterate with mutable values
for key in map.keys() { }         // Keys only
for value in map.values() { }     // Values only
for value in map.values_mut() { } // Mutable values
 
INFO

map.len();                        // Number of entries
map.is_empty();                   // Check if empty


// Count word occurrences
let count = map.entry(word).or_insert(0);
*count += 1;

// Update existing or insert new
map.entry(key)
    .and_modify(|v| *v += 10)     // If exists, modify
    .or_insert(50);               // If not, insert 50


COMMON PATTERNS

// Get with default
let value = map.get(&key).unwrap_or(&default_value);

// Update if exists
if let Some(value) = map.get_mut(&key) {
    *value += 1;
}

// Insert only if new
map.entry(key).or_insert_with(|| expensive_computation());



 */

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mange"), 2);

    // TODO: Put more fruits in your basket.

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
