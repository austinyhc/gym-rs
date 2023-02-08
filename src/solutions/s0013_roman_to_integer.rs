/**
 * [13] Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given a roman numeral, convert it to an integer.
 *
 * <strong class="example">Example 1:
 *
 * Input: s = "III"
 * Output: 3
 * Explanation: III = 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 15
 * 	s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
 * 	It is guaranteed that s is a valid roman numeral in the range [1, 3999].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/roman-to-integer/
// discuss: https://leetcode.com/problems/roman-to-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut acc = 0;
        let mut stream = s.chars().peekable();
        while let Some(curr) = stream.next() {
            let next = stream.peek();
            acc += match (curr, next) {
                ('I', Some('V')) => -1,
                ('I', Some('X')) => -1,
                ('X', Some('L')) => -10,
                ('X', Some('C')) => -10,
                ('C', Some('D')) => -100,
                ('C', Some('M')) => -100,
                ('I', _) => 1,
                ('V', _) => 5,
                ('X', _) => 10,
                ('L', _) => 50,
                ('C', _) => 100,
                ('D', _) => 500,
                ('M', _) => 1000,
                _ => panic!("Bad symbol!")
            }
        }
        acc
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(3, Solution::roman_to_int(format!("III")));
        assert_eq!(58, Solution::roman_to_int(format!("LVIII")));
        assert_eq!(1994, Solution::roman_to_int(format!("MCMXCIV")));
        assert_eq!(9, Solution::roman_to_int(format!("IX")));
        assert_eq!(19, Solution::roman_to_int(format!("IXX")));
    }
}

