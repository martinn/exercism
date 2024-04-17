use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let word_anagram = create_hash_set(&word);

    for possible_anagram in possible_anagrams.iter() {
        if word.to_lowercase() == possible_anagram.to_lowercase() {
            continue;
        }
        let anagram = create_hash_set(&possible_anagram);
        if anagram == word_anagram {
            result.insert(&possible_anagram);
        }
    }

    result
}

fn create_hash_set(word: &str) -> HashMap<char, i32> {
    word.to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut hset, c| {
            *hset.entry(c).or_insert(0) += 1;
            hset
        })
}
