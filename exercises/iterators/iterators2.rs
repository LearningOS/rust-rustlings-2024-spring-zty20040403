// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



pub fn capitalize_first(input: &str) -> String {
    if let Some(first) = input.chars().next() {
        let mut capitalized = first.to_uppercase().to_string();
        capitalized.push_str(input.chars().skip(1).collect::<String>().as_str());
        capitalized
    } else {
        String::new()
    }
}


pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
}


pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&word| capitalize_first(word)).collect::<Vec<_>>().join("")
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