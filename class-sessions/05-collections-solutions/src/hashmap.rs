use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
fn get_most_common_words(file_path: &str) -> String {
    let file_contents = fs::read_to_string(file_path)
        .expect("File should exist")
        .replace(&['?', '.', '!', '-'], "");
    ///////////////////////// IMPERATIVE SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    // let mut word_frequencies = HashMap::new();

    // for word in file_contents.split_whitespace() {
    //     word_frequencies
    //         .entry(word)
    //         .and_modify(|counter| *counter += 1)
    //         .or_insert(1);
    // }

    // let mut max_word: &str = &String::new();
    // let mut max_word_frequency = 0;

    // for (word, freq) in word_frequencies.clone().into_iter() {
    //     if freq > max_word_frequency {
    //         max_word = word;
    //         max_word_frequency = freq;
    //     }
    // }
    // println!("{:?}", word_frequencies.clone());

    // max_word.to_owned()

    ////////////////////////// FUNCTIONAL SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    file_contents
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            let _ = *acc
                .entry(word.to_lowercase())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        })
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, _)| word)
        .expect("Most common word should exist")
        .to_owned()
}

#[allow(dead_code)]
fn get_unique_characters(input: String) -> Vec<char> {
    ///////////////////////// IMPERATIVE SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    // let mut letter_frequencies = HashMap::new();
    // for char in input.chars() {
    //     letter_frequencies
    //         .entry(char)
    //         .and_modify(|counter| *counter += 1)
    //         .or_insert(1);
    // }

    // let mut single_chars = Vec::new();
    // for (char, freq) in letter_frequencies.into_iter() {
    //     if freq == 1 {
    //         single_chars.push(char);
    //     }
    // }
    // single_chars

    ////////////////////////// FUNCTIONAL SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    input
        .chars()
        .fold(HashMap::new(), |mut acc, char| {
            let _ = *acc
                .entry(char)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        })
        .into_iter()
        .filter_map(|(char, freq)| if freq == 1 { Some(char) } else { None })
        .collect()
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use crate::hashmap::{get_most_common_words, get_unique_characters};

    #[test]
    fn test_get_most_common_words() {
        assert_eq!(get_most_common_words("poems/poem.txt"), String::from("to"));
        assert_eq!(
            get_most_common_words("poems/a_line_storm_song.txt"),
            String::from("the")
        );
        assert_eq!(
            get_most_common_words("poems/dream_variations.txt"),
            String::from("the")
        );
        assert_eq!(
            get_most_common_words("poems/haiku.txt"),
            String::from("candle")
        );
    }

    #[test]
    fn test_get_unique_characters() {
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("strawberry")),
            &mut vec!['s', 't', 'a', 'w', 'b', 'e', 'y']
        ));
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("black")),
            &mut vec!['b', 'l', 'a', 'c', 'k']
        ));
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("white")),
            &mut vec!['w', 'h', 'i', 't', 'e']
        ));
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("none")),
            &mut vec!['o', 'e']
        ));
    }

    fn check_unordered_vec_are_eq(vec1: &mut Vec<char>, vec2: &mut Vec<char>) -> bool {
        vec1.sort();
        vec2.sort();

        vec1 == vec2
    }
}
