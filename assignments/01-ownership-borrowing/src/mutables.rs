/// Increments a mutable i32 reference by 1. On overflow, wraps around.
fn increments() {
    todo!();
}

/// Given two mutable i32 references, swap their values
fn swap() {
    todo!();
}

/// Given a mutable string slice, strip all vowels and digits
fn strip_vowels_and_digits() {
    todo!();
}

// DO NOT MODIFY BELOW THIS LINE

// Helper functions

fn strip_vowels(s: &mut String) {
    let vowels = "aeiouAEIOU";
    s.retain(|c| !vowels.contains(c));
}

fn strip_digits(mut s: String) -> String {
    let digits = "0123456789";
    s.retain(|c| !digits.contains(c));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increments() {
        let mut x = 0;
        increments(&mut x);
        assert_eq!(x, 1);
    }

    #[test]
    fn test_increments_negative() {
        let mut x = -1;
        increments(&mut x);
        assert_eq!(x, 0);
    }

    #[test]
    fn test_increments_overflow() {
        let mut x = i32::MAX;
        increments(&mut x);
        assert_eq!(x, i32::MIN);
    }

    #[test]
    fn test_swap() {
        let mut x = 1;
        let mut y = 2;
        swap(&mut x, &mut y);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
    }

    #[test]
    fn test_strip_vowels_and_digits() {
        let mut s = String::from("Hello, 123!");
        let s = strip_vowels_and_digits(&mut s);
        assert_eq!(s, "Hll, !");
    }

    #[test]
    fn test_strip_vowels_and_digits_empty() {
        let mut s = String::from("");
        let s = strip_vowels_and_digits(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_strip_vowels_and_digits_no_vowels_or_digits() {
        let mut s = String::from("dk hw t wrt ths");
        let s = strip_vowels_and_digits(&mut s);
        assert_eq!(s, "dk hw t wrt ths");
    }
}
