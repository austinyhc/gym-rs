/**
 * [33] Search in Rotated Sorted Array
 *
 * There is an integer array nums sorted in ascending order (with distinct values).
 * Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
 * Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * <strong class="example">Example 2:
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 * <strong class="example">Example 3:
 * Input: nums = [1], target = 0
 * Output: -1
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5000
 * 	-10^4 <= nums[i] <= 10^4
 * 	All values of nums are unique.
 * 	nums is an ascending array that is possibly rotated.
 * 	-10^4 <= target <= 10^4
 * 
 */
use crate::solutions::s0153_find_minimum_in_rotated_sorted_array::Solution;

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        let offset: i32 = Self::find_first_of_rotated_sorted_array(&nums) as i32;

        let length: i32 = nums.len() as i32;
        let mut left: i32 = -1;
        let mut right: i32 = nums.len() as i32;

        while left+1 != right {
            let mid = left + (right-left) / 2;
            let index = (mid+offset) % length;
            let num = nums[index as usize];

            if num == target {
                return index;
            } else if target < num {
                right = mid;
            } else {
                left = mid;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(4, Solution::search(vec![4,5,6,7,0,1,2], 0));
        assert_eq!(-1, Solution::search(vec![4,5,6,7,0,1,2], 3));
        assert_eq!(-1, Solution::search(vec![1], 3));
        assert_eq!(0, Solution::search(vec![1,2], 1));
        assert_eq!(1, Solution::search(vec![1,2], 2));
    }
}

