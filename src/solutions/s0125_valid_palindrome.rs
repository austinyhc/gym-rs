/**
 * [125] Valid Palindrome
 *
 * A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
 * Given a string s, return true if it is a palindrome, or false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = " "
 * Output: true
 * Explanation: s is an empty string "" after removing non-alphanumeric characters.
 * Since an empty string reads the same forward and backward, it is a palindrome.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 2 * 10^5
 * 	s consists only of printable ASCII characters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome/
// discuss: https://leetcode.com/problems/valid-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {

        let stream = s.chars().collect::<Vec<_>>();
        if  stream.len() == 1 { return true; }

        let mut head = 0;
        let mut tail = stream.len() - 1;

        while head < tail {
            if !stream[head].is_alphabetic() { head += 1; continue; }
            if !stream[tail].is_alphabetic() { tail -= 1; continue; }

            if stream[head].to_ascii_lowercase() != stream[tail].to_ascii_lowercase()
            { return false; }
            head += 1;
            tail -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(true, Solution::is_palindrome(format!(" ")));
        assert_eq!(true, Solution::is_palindrome(format!("a")));
        assert_eq!(true, Solution::is_palindrome(format!("A man, a plan, a canal: Panama")));
        assert_eq!(false, Solution::is_palindrome(format!("race a car")));
        assert_eq!(true, Solution::is_palindrome(format!("Sir, I demand, I am a maid named Iris.")));
        assert_eq!(true, Solution::is_palindrome(format!("Red roses run no risk, sir, on Nurse’s order.")));
    }
}

