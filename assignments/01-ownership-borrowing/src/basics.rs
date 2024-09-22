/// Take ownership of the passed in string and print it
fn takes_ownership_and_prints(s: String) {
    println!("{s}");
}

/// Take a string slice and return the first letter, or None in the case
/// of a blank string
fn first_letter(s: &str) -> Option<&str> {
    match s.chars().next() {
        Some(c) => Some(&s[..c.len_utf8()]),
        None => None
    }
}

type Student = (String, u32);

/// Given a reference to a student, return the student's name
fn get_name(student: &Student) -> String {
    student.0.clone()
}

/// Given a slice of i32s, return the sum of the elements
fn slice_sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

/// Given a string slice, look for a substring, and return a slice of the first
/// occurrence of the substring (return None if the substring is not found)
/// 
/// Lifetime of the returned slice is the same as the input string
fn find_in_string<'a>(s: &'a str, sub: &str) -> Option<&'a str> {
    let match_byte = s.find(sub);
    match match_byte {
        Some(x) => Some(&s[x..x+sub.len()]),
        None => None
    }
}

// DO NOT MODIFY BELOW THIS LINE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takes_ownership_and_prints() {
        let s = String::from("hello");
        takes_ownership_and_prints(s);
    }

    #[test]
    fn test_get_first_letter() {
        let s = String::from("hello");
        assert_eq!(first_letter(&s), Some("h"));
    }

    #[test]
    fn test_get_first_letter_empty_string() {
        let s = String::from("");
        assert_eq!(first_letter(&s), None);
    }

    #[test]
    fn test_get_name() {
        let student = (String::from("Alice"), 20);
        assert_eq!(get_name(&student), "Alice");
    }

    #[test]
    fn test_slice_sum() {
        let slice = [1, 2, 3, 4, 5];
        assert_eq!(slice_sum(&slice), 15);
    }

    #[test]
    fn test_slice_sum_empty() {
        let slice = [];
        assert_eq!(slice_sum(&slice), 0);
    }

    #[test]
    fn test_find_in_string() {
        let sentence = "The quick brown fox jumps over the lazy dog";
        let word = String::from("fox");
        let found = find_in_string(sentence, &word);
        drop(word);
        assert_eq!(found, Some("fox"));
    }
}
