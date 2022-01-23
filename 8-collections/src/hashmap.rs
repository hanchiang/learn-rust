use std::collections::HashMap;

pub fn main() {
    create_hashmap();
    accessing_values();
    updating();
}

fn create_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn create_hashmap_with_vectors() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures
    // and Rust doesn’t know which you want unless you specify
    let mut scores :HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn accessing_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("key: {}, value: {}", team_name, score.unwrap_or(&0));

    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
}

fn updating() {
    let mut scores = HashMap::new();

    // overwrite a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // insert a value if the key is not present
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // updating a value based on old vlaue
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
