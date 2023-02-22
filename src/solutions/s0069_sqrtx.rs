/**
 * [69] Sqrt(x)
 *
 * Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
 * You must not use any built-in exponent function or operator.
 * 
 * 	For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: x = 4
 * Output: 2
 * Explanation: The square root of 4 is 2, so we return 2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: x = 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= x <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sqrtx/
// discuss: https://leetcode.com/problems/sqrtx/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        
        if x < 2 { return x; }

        let mut left = 0;
        let mut right = x/2 + 1;
        
        while left+1 != right {
            let mid = left + (right - left)/2;
            let num = mid * mid;
            if num <= x {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(1, Solution::my_sqrt(3));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
        assert_eq!(11, Solution::my_sqrt(136));
    }
}

