/**
 * [392] Is Subsequence
 *
 * Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
 * A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>" while "aec" is not).
 *
 * <strong class="example">Example 1:
 * Input: s = "abc", t = "ahbgdc"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "axc", t = "ahbgdc"
 * Output: false
 *
 * Constraints:
 *
 * 	0 <= s.length <= 100
 * 	0 <= t.length <= 10^4
 * 	s and t consist only of lowercase English letters.
 *
 *
 * Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 10^9, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/is-subsequence/
// discuss: https://leetcode.com/problems/is-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t_chars = t.chars();
        for sc in s.chars() {
            loop {
                if let Some(tc) = t_chars.next() {
                    if tc == sc { break; }
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn _is_subsequence(s: String, t: String) -> bool {
        let n = t.len();
        let mut i = 0;

        for c in s.chars() {
            while i < n && t.chars().nth(i).unwrap() != c {
                i += 1;
            }

            if i == n { return false; }
            i += 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392() {
        assert_eq!(true, Solution::is_subsequence(format!("abc"), format!("ahbgdc")));
        assert_eq!(false, Solution::is_subsequence(format!("axc"), format!("agbgdc")));
        assert_eq!(true, Solution::is_subsequence(format!(""), format!("agbgdc")));
        assert_eq!(false, Solution::is_subsequence(format!("abc"), format!("")));
        assert_eq!(true, Solution::is_subsequence(format!("b"), format!("abc")));
    }
}

