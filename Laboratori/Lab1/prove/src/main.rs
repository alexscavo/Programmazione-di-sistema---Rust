use std::cmp::Ordering;
use std::collections::BTreeMap;

impl Ord for BTreeMap<i32, &str> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            return Ordering::Greater;
        }

        return Ordering::Less;
    }
}

fn main() {
    let mut map = BTreeMap::new();
    map.insert(3, "tre");
    map.insert(1, "uno");
    map.insert(4, "quattro");
    map.insert(2, "due");
    map.insert(5, "cinque");

    println!("Mappa: {:?}", map);

    // Iterazione sugli elementi della mappa
    println!("Iterazione sulla mappa:");
    for (key, value) in &map {
        println!("Chiave: {}, Valore: {}", key, value);
    }
}