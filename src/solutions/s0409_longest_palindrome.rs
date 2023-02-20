/**
 * [409] Longest Palindrome
 *
 * Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
 * Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "abccccdd"
 * Output: 7
 * Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "a"
 * Output: 1
 * Explanation: The longest palindrome that can be built is "a", whose length is 1.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 2000
 * 	s consists of lowercase and/or uppercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindrome/
// discuss: https://leetcode.com/problems/longest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {

        let mut frequencies = [0i32; 128];

        for &byte in s.as_bytes() {
            frequencies[byte as usize] += 1;
        }
        
        let mut has_unique = false;
        let mut sum = 0i32;
        for f in frequencies {
            if !has_unique && f % 2 == 1 {
                has_unique = true;
            }
            sum += f / 2;
        }
        2*sum + if has_unique {1} else {0}
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_409() {
        assert_eq!(7, Solution::longest_palindrome("abcccccdd".to_string()));
        assert_eq!(1, Solution::longest_palindrome("a".to_string()));
        assert_eq!(2, Solution::longest_palindrome("aa".to_string()));
        assert_eq!(10, Solution::longest_palindrome("bbbbbbbbbb".to_string()));
        assert_eq!(11, Solution::longest_palindrome("bbbbbbbbbbb".to_string()));
    }
}

