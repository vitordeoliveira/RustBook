use std::collections::HashMap;

fn main() {
    // Storing Keys with Associated Values in Hash Maps
    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String,
    // the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // map.insert(&field_name, &field_value);

    // drop(field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    for (key, value) in &map {
        println!("{key}: {value}");
    }
    // println!("{field_name}: {field_value}");

    // Updating a Hash Map

    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // Hashing Functions
    // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.
    // This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
    // If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
    // A hasher is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10.
    // You don’t necessarily have to implement your own hasher from scratch;
    // crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let mut sum = 0;

    for n in &v {
        sum += n;
    }

    println!("{sum}");

    let mut sort_v = v.clone();
    sort_v.sort();
    println!("sort_v is {:?}", sort_v);
    println!("Median is {}", sort_v[sort_v.len() / 2]);
}
