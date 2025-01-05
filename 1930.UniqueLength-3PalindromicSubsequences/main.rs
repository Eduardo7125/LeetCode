// 1930. Unique Length-3 Palindromic Subsequences

// Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
// Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.
// A palindrome is a string that reads the same forwards and backwards.
// A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted
// without changing the relative order of the remaining characters.
// For example, "ace" is a subsequence of "abcde".
 
// Example 1:

// Input: s = "aabca"
// Output: 3
// Explanation: The 3 palindromic subsequences of length 3 are:
// - "aba" (subsequence of "aabca")
// - "aaa" (subsequence of "aabca")
// - "aca" (subsequence of "aabca")
// Example 2:

// Input: s = "adc"
// Output: 0
// Explanation: There are no palindromic subsequences of length 3 in "adc".
// Example 3:

// Input: s = "bbcbaba"
// Output: 4
// Explanation: The 4 palindromic subsequences of length 3 are:
// - "bbb" (subsequence of "bbcbaba")
// - "bcb" (subsequence of "bbcbaba")
// - "bab" (subsequence of "bbcbaba")
// - "aba" (subsequence of "bbcbaba")
 

// Constraints:

// 3 <= s.length <= 105
// s consists of only lowercase English letters.

use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut first_occurrence = vec![-1; 26];
        let mut last_occurrence = vec![-1; 26];

        // Rastrear los primeros y últimos índices de cada carácter
        for (i, &c) in chars.iter().enumerate() {
            let index = (c as u8 - b'a') as usize;
            if first_occurrence[index] == -1 {
                first_occurrence[index] = i as i32;
            }
            last_occurrence[index] = i as i32;
        }

        let mut result = 0;

        // Para cada carácter, contar los caracteres únicos entre su primera y última aparición
        for i in 0..26 {
            if first_occurrence[i] != -1 && last_occurrence[i] != -1 && first_occurrence[i] < last_occurrence[i] {
                let start = first_occurrence[i] as usize;
                let end = last_occurrence[i] as usize;

                let mut unique_chars = HashSet::new();
                for j in (start + 1)..end {
                    unique_chars.insert(chars[j]);
                }

                result += unique_chars.len();
            }
        }

        result as i32
    }
}
