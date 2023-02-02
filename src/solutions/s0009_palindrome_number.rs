/**
 * [9] Palindrome Number
 *
 * Given an integer x, return true if x is a <span data-keyword="palindrome-integer">palindrome</span>, and false otherwise.
 *
 * <strong class="example">Example 1:
 *
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 * <strong class="example">Example 2:
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 *
 * <strong class="example">Example 3:
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 *
 *
 * Constraints:
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 *
 * Follow up: Could you solve it without converting the integer to a string?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-number/
// discuss: https://leetcode.com/problems/palindrome-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        let mut x = x;

        if x < 0 { return false; }
        if x == 0 { return true;  }

        let ndigits = x.ilog10() + 1;
        let mut t = 10i32.pow(ndigits-1);

        while t != 0 {
            let a = x / t;
            let b = x % 10;
            if a != b { return false; }
            x = (x - a*t) / 10;
            t /= 100;
        }
        true
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(true, Solution::is_palindrome(0));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(true,  Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(10));
    }
}

