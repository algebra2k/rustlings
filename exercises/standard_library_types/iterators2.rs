// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut s: String = String::new();
    match c.next() {
        None => String::new(),
        Some(first) => {
            s.push(first.to_ascii_uppercase());
            while let Some(v) = c.next() {
                s.push(v);
            }

            s
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut words_vec: Vec<String> = Vec::new();
    words_vec.reserve(words.len());

    for i in words.iter() {
        let mut c = (*i).chars();
        match c.next() {
            None => {}
            Some(first) => {
                let mut s = String::new();
                s.push(first.to_ascii_uppercase());
                while let Some(v) = c.next() {
                    s.push(v);
                }

                words_vec.push(s);
            }
        }
    }

    words_vec
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut s = String::new();
    for i in words.iter() {
        let mut c = (*i).chars();
        let new_str = match c.next() {
            None => String::from(*i),
            Some(first) => {
                let mut s = String::new();
                s.push(first.to_ascii_uppercase());
                while let Some(v) = c.next() {
                    s.push(v);
                }
                s
            }
        };
        s.push_str(new_str.as_str());
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
