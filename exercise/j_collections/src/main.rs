use std::collections::HashMap;

fn main() {
    // The text below is an exerpt from the book `Green Eggs & Ham` by Dr. Seuss with punctuation
    // removed.
    //
    // `vec` is a macro that lets you define a Vec literal.
    let poem = vec![
        "I do not like them in a box",
        "I do not like them with a fox",
        "I do not like them in a house",
        "I do not like them with a mouse",
        "I do not like them here or there",
        "I do not like them anywhere",
        "I do not like green eggs and ham",
    ];

    // 1. At runtime, we decide to add another line to the end of the poem.
    // Add `next_line` below to end of the `poem` Vec above by using the `.push` method
    //
    // Hint: `poem` needs to be mutable for this to work.
    let next_line = "I do not like them Sam I am";

    let mut word_map: HashMap<&str, u32> = HashMap::new();
    for word in poem.split_whitespace() {
        if word_map.contains_key(word) {
            let count = word_map.get_mut(word).unwrap();
            *count += 1;
        } else {
            word_map.insert(word, 1);
        }
    }

    let mut sorted_keys = word_map.keys().collect::<Vec<_>>();
    sorted_keys.sort();
    for key in sorted_keys {
        let value = word_map.get(key).unwrap();
        println!("{key} {value}");
    }
}
