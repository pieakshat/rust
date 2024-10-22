use std::hash::{Hash, Hasher}; 
use std::collections::hash_map::DefaultHasher; 

const SIZE: usize = 100; 

struct HashTable<K, V> {
    buckets: Vec<Option<(K, V)>>,  
}

impl<K: Eq + Hash, V> HashTable<K, V> {

    fn new() -> HashTable<K, V> {
        let mut buckets = Vec::with_capacity(SIZE); 
        buckets.resize_with(SIZE, || None); 
        
        HashTable {
            buckets, 
        }
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new(); 
        key.hash(&mut hasher); 
        (hasher.finish() as usize) % SIZE 
    }

    fn insert(&mut self, key: K, value: V) {
        let mut index = self.get_index(&key); 

        while let Some((existing_key, _)) = &self.buckets[index] {
            if existing_key == &key {
                self.buckets[index] = Some((key, value)); 
                return; 
            }
            index = (index + 1) % SIZE; 
        }

        self.buckets[index] = Some((key, value)); 
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.get_index(key); 

        while let Some((existing_key, value)) = &self.buckets[index] {
            if existing_key == key {
                return Some(value); 
            }
            index = (index + 1) % SIZE; 
        }
        None
    }
}

fn main() {
    let mut table = HashTable::new(); 

    table.insert("apple", 1); 
    table.insert("banana", 2); 
    table.insert("pear", 3);  

    println!("apple: {:?}", table.get(&"apple")); 
    println!("banana: {:?}", table.get(&"banana")); 
    println!("pear: {:?}", table.get(&"pear")); 
}
