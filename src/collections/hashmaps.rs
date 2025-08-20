use std::collections::HashMap;

pub fn run() {
    create();
    ownership();
    overwriting();
    updating_from_old_value();
}

fn updating_from_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}

fn overwriting() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{scores2:?}");
}

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are moved into the map
    // and can no longer be used
    // below won't compile
    // println!("{}: {}", field_name, field_value);

    let field_name2 = String::from("Favorite food");
    let field_value2 = String::from("Pizza");
    let mut ref_map = HashMap::new();
    ref_map.insert(&field_name2, &field_value2);

    // with ref types can still use afterward
    println!("{}: {}", field_name2, field_value2);

}

fn create() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}'s score is {}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}