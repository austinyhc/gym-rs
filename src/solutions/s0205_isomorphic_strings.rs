/**
 * [205] Isomorphic Strings
 *
 * Given two strings s and t, determine if they are isomorphic.
 * Two strings s and t are isomorphic if the characters in s can be replaced to get t.
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
 *
 * <strong class="example">Example 1:
 * Input: s = "egg", t = "add"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "foo", t = "bar"
 * Output: false
 * <strong class="example">Example 3:
 * Input: s = "paper", t = "title"
 * Output: true
 *
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	t.length == s.length
 * 	s and t consist of any valid ascii character.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/isomorphic-strings/
// discuss: https://leetcode.com/problems/isomorphic-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s2t: HashMap<char, char> = HashMap::new();
        let mut t2s: HashMap<char, char> = HashMap::new();

        let mut is = s.chars();
        let mut it = t.chars();

        while let (Some(char_s), Some(char_t)) = (is.next(), it.next()) {

            if let Some(&t_) = s2t.get(&char_s) {
                if t_ != char_t { return false; }
            } else {
                s2t.insert(char_s, char_t);
            }

            if let Some(&s_) = t2s.get(&char_t) {
                if s_ != char_s { return false; }
            } else {
                t2s.insert(char_t, char_s);
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {
        assert_eq!(true, Solution::is_isomorphic(format!("egg"), format!("add")));
        assert_eq!(false, Solution::is_isomorphic(format!("foo"), format!("bar")));
        assert_eq!(true, Solution::is_isomorphic(format!("paper"), format!("title")));
    }
}

