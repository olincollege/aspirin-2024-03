use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
fn get_most_common_words(file_path: &str) -> String {
    todo!()
}

#[allow(dead_code)]
fn get_unique_characters(input: String) -> Vec<char> {
    todo!()
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
