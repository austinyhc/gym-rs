/**
 * [70] Climbing Stairs
 *
 * You are climbing a staircase. It takes n steps to reach the top.
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 2
 * Output: 2
 * Explanation: There are two ways to climb to the top.
 * 1. 1 step + 1 step
 * 2. 2 steps
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3
 * Output: 3
 * Explanation: There are three ways to climb to the top.
 * 1. 1 step + 1 step + 1 step
 * 2. 1 step + 2 steps
 * 3. 2 steps + 1 step
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 45
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/climbing-stairs/
// discuss: https://leetcode.com/problems/climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {

    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = HashMap::<i32, i32>::new();
        Self::dfs(n, &mut memo)
    }

    pub fn dfs(n: i32, memo: &mut HashMap<i32,i32>) -> i32 {

        if let Some(&value) = memo.get(&n) {
            return value;
        }

        let mut num_of_ways = 0i32;
        if n == 0 { return 1; }
        if n >= 1 { num_of_ways += Self::dfs(n-1, memo); }
        if n >= 2 { num_of_ways += Self::dfs(n-2, memo); }
        memo.insert(n, num_of_ways);
        num_of_ways
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(1346269, Solution::climb_stairs(30));
        assert_eq!(1134903170, Solution::climb_stairs(44));
    }
}

