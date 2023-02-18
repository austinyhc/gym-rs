/**
 * [167] Two Sum II - Input Array Is Sorted
 *
 * Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
 * Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
 * The tests are generated such that there is exactly one solution. You may not use the same element twice.
 * Your solution must use only constant extra space.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: numbers = [<u>2</u>,<u>7</u>,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: numbers = [<u>2</u>,3,<u>4</u>], target = 6
 * Output: [1,3]
 * Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: numbers = [<u>-1</u>,<u>0</u>], target = -1
 * Output: [1,2]
 * Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
 * 
 *  
 * Constraints:
 * 
 * 	2 <= numbers.length <= 3 * 10^4
 * 	-1000 <= numbers[i] <= 1000
 * 	numbers is sorted in non-decreasing order.
 * 	-1000 <= target <= 1000
 * 	The tests are generated such that there is exactly one solution.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// discuss: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let sum = numbers[i] + numbers[j];
            match sum.cmp(&target) {
                Ordering::Greater => j -= 1,
                Ordering::Less    => i += 1,
                Ordering::Equal   => return vec![(i+1) as i32, (j+1) as i32],
            }
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_167() {
        assert_eq!(vec![1,2], Solution::two_sum_2(vec![2,7,11,15], 9));
        assert_eq!(vec![1,3], Solution::two_sum_2(vec![2,3,4,6], 6));
        assert_eq!(vec![1,2], Solution::two_sum_2(vec![-1,0], -1));
    }
}

