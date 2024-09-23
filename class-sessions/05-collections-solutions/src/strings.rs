#[allow(dead_code)]
fn merge_strings(arr: Vec<String>) -> String {
    ///////////////////////// IMPERATIVE SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    // let mut res = String::new();
    // for elem in arr {
    //     res.push_str(&elem);
    // }

    // res

    ////////////////////////// FUNCTIONAL SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    arr.into_iter().fold(String::new(), |mut acc, curr| {
        acc.push_str(&curr);
        acc
    })
}

#[allow(dead_code)]
fn count_letter(word: String, letter: char) -> u8 {
    ///////////////////////// IMPERATIVE SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    // let mut count = 0;
    // for chr in word.chars() {
    //     if chr == letter {
    //         count += 1;
    //     }
    // }

    // count

    ////////////////////////// FUNCTIONAL SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    word.chars().fold(0, |count, chr| match chr == letter {
        true => count + 1,
        false => count,
    })
}

#[allow(dead_code)]
fn selectively_capitalize(input: String, idx_to_capitalize: Vec<usize>) -> String {
    ///////////////////////// IMPERATIVE SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    // let mut ret = String::new();
    // for (idx, val) in input.chars().enumerate() {
    //     match idx_to_capitalize.contains(&idx) {
    //         true => ret.push_str(&val.to_uppercase().to_string()),
    //         false => ret.push(val),
    //     }
    // }

    // ret

    ////////////////////////// FUNCTIONAL SOLUTION \\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    input
        .chars()
        .enumerate()
        .map(|(idx, val)| match idx_to_capitalize.contains(&idx) {
            true => val.to_uppercase().to_string(),
            false => val.to_string(),
        })
        .collect()
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use crate::strings::{count_letter, merge_strings, selectively_capitalize};

    #[test]
    fn test_merge_strings() {
        assert_eq!(
            merge_strings(vec![
                String::from("hello"),
                String::from(" "),
                String::from("world"),
                String::from("!")
            ]),
            "hello world!"
        );

        assert_eq!(
            merge_strings(vec![
                String::from("writing"),
                String::from(" "),
                String::from("tests"),
                String::from(" is annoy"),
                String::from("ing!!!!"),
            ]),
            "writing tests is annoying!!!!"
        );

        assert_eq!(
            merge_strings(vec![String::from("very annoying...")]),
            "very annoying..."
        );
    }

    #[test]
    fn test_count_letter() {
        assert_eq!(count_letter(String::from("strawberry"), 'r'), 3);
        assert_eq!(count_letter(String::from("blueberry"), 'b'), 2);
        assert_eq!(count_letter(String::from("raspberry"), 'y'), 1);
        assert_eq!(count_letter(String::from("blackberry"), 'z'), 0);
    }

    #[test]
    fn test_selectively_capitalize() {
        assert_eq!(
            selectively_capitalize(String::from("hello"), vec![1, 3]),
            String::from("hElLo")
        );

        assert_eq!(
            selectively_capitalize(String::from("aggressive hello"), (0..16).collect()),
            String::from("AGGRESSIVE HELLO")
        );

        assert_eq!(
            selectively_capitalize(String::from("goodbye"), vec![0, 2, 4, 6]),
            String::from("GoOdByE")
        );

        assert_eq!(
            selectively_capitalize(String::from("silent goodbye"), Vec::new()),
            String::from("silent goodbye")
        );
    }
}
