/**
 * [7] Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *
 * <strong class="example">Example 1:
 *
 * Input: x = 123
 * Output: 321
 *
 * <strong class="example">Example 2:
 *
 * Input: x = -123
 * Output: -321
 *
 * <strong class="example">Example 3:
 *
 * Input: x = 120
 * Output: 21
 *
 *
 * Constraints:
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/
// discuss: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    // pub fn reverse(x: i32) -> i32 {
    //     let mut r_str = String::new();
    //     let r_num: i32;
    //
    //     for c in x.abs().to_string().chars().rev() {
    //         r_str.push(c);
    //     }
    //     r_num = match r_str.parse() {
    //         Ok(n) => n,
    //         Err(_) => { return 0; }
    //     };
    //     r_num * x.signum()
    // }

    pub fn reverse(x: i32) -> i32 {
        let mut result = 0;
        let sign = x.signum(); 
        Self::reverser(x.abs(), &mut result);
        result * sign
    }

    pub fn reverser(x: i32, result: &mut i32) {
        if x == 0 { return; }

        *result = match result.checked_mul(10) {
            Some(n) => n,
            None => { *result = 0; return; }
        };
        *result += x % 10;

        Self::reverser(x / 10, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(21, Solution::reverse(120));
        assert_eq!(0, Solution::reverse(123_456_789_9));
    }
}

