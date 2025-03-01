/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // Normalize the strings by removing non-alphabetical characters and converting to lowercase
    let normalize = |s: &String| -> String {
        s.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect()
    };

    let normalized_s1 = normalize(&s1);
    let normalized_s2 = normalize(&s2);

    // Check if the lengths are the same
    if normalized_s1.len() != normalized_s2.len() {
        return false;
    }

    // Count the frequency of each character
    let mut char_count = HashMap::new();

    for c in normalized_s1.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    for c in normalized_s2.chars() {
        let count = char_count.entry(c).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
