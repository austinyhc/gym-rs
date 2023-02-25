/**
 * [344] Reverse String
 *
 * Write a function that reverses a string. The input string is given as an array of characters s.
 * You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 *  
 * <strong class="example">Example 1:
 * Input: s = ["h","e","l","l","o"]
 * Output: ["o","l","l","e","h"]
 * <strong class="example">Example 2:
 * Input: s = ["H","a","n","n","a","h"]
 * Output: ["h","a","n","n","a","H"]
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s[i] is a <a href="https://en.wikipedia.org/wiki/ASCII#Printable_characters" target="_blank">printable ascii character</a>.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-string/
// discuss: https://leetcode.com/problems/reverse-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {

        let l = s.len();

        for i in 0..l / 2 {
            s.swap(i, l - i - 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_344() {
        let mut test = vec!['h','e','l','l','o'];
        Solution::reverse_string(&mut test);
        assert_eq!(vec!['o','l','l','e','h'], test);

        let mut test = vec!['H','a','n','n','a','h'];
        Solution::reverse_string(&mut test);
        assert_eq!(vec!['h','a','n','n','a','H'], test);
    }
}

