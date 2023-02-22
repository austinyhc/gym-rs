/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
 * If target is not found in the array, return [-1, -1].
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * <strong class="example">Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * <strong class="example">Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	nums is a non-decreasing array.
 * 	-10^9 <= target <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut ret = vec![-1, -1];

        if nums.len() == 0 { return ret; }

        let mut left: i32 = -1;
        let mut right: i32 = nums.len() as i32;
        
        while left+1 != right {
            let mid = left + (right-left)/2;
            if nums[mid as usize] >= target {
                right = mid;
            } else {
                left = mid;
            }
        }
        if right == nums.len() as i32 || nums[right as usize] != target {
            return ret;
        }
        ret[0] = right;

        let mut left: i32 = right-1;
        let mut right: i32 = nums.len() as i32;

        while left+1 != right {
            let mid = left + (right-left)/2;
            if nums[mid as usize] <= target {
                left = mid;
            } else {
                right = mid;
            }
        }
        ret[1] = left;
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(vec![3,4], Solution::search_range(vec![5,7,7,8,8,10], 8));
        assert_eq!(vec![-1,-1], Solution::search_range(vec![5,7,7,8,8,10], 6));
        assert_eq!(vec![-1,-1], Solution::search_range(vec![], 0));
        assert_eq!(vec![-1,-1], Solution::search_range(vec![2,2], 3));
    }
}

