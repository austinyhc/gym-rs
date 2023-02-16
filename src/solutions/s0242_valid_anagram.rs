/**
 * [242] Valid Anagram
 *
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * <strong class="example">Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *  
 * Constraints:
 * 
 * 	1 <= s.length, t.length <= 5 * 10^4
 * 	s and t consist of lowercase English letters.
 * 
 *  
 * Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-anagram/
// discuss: https://leetcode.com/problems/valid-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut dict = [0u32; 26];

        for c in s.into_bytes() {
            let index = c - b'a';
            dict[index as usize] += 1;
        }

        for c in t.into_bytes() {
            let index = c - b'a';

            if dict[index as usize] == 0 { return false; }
            dict[index as usize] -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert_eq!(true, Solution::is_anagram(format!("anagram"), format!("nagaram")));
        assert_eq!(false, Solution::is_anagram(format!("rat"), format!("car")));
    }
}

