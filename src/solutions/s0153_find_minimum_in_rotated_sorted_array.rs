/**
 * [153] Find Minimum in Rotated Sorted Array
 *
 * Suppose an array of length n sorted in ascending order is rotated between 1 and n times.
* For example, the array nums = [0,1,2,4,5,6,7] might become:
 * 
 * 	[4,5,6,7,0,1,2] if it was rotated 4 times.
 * 	[0,1,2,4,5,6,7] if it was rotated 7 times.
 * 
 * Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
 * Given the sorted rotated array nums of unique elements, return the minimum element of this array.
 * You must write an algorithm that runs in O(log n) time.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [3,4,5,1,2]
 * Output: 1
 * Explanation: The original array was [1,2,3,4,5] rotated 3 times.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [4,5,6,7,0,1,2]
 * Output: 0
 * Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [11,13,15,17]
 * Output: 11
 * Explanation: The original array was [11,13,15,17] and it was rotated 4 times. 
 * 
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 5000
 * 	-5000 <= nums[i] <= 5000
 * 	All the integers of nums are unique.
 * 	nums is sorted and rotated between 1 and n times.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
// discuss: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {

        let pos = Self::find_first_of_rotated_sorted_array(&nums);
        nums[pos]
    }

    pub fn find_first_of_rotated_sorted_array(nums: &Vec<i32>) -> usize {
        let mut left: i32 = -1;
        let mut right: i32 = nums.len() as i32;
        
        if nums[(right-1) as usize] >= nums[0] {
            return 0;
        }

        while left+1 != right {
            let mid = left + (right-left)/2;
            if nums[mid as usize] < nums[0] {
                right = mid;
            } else {
                left = mid;
            }
        }
        right as usize
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_153() {
        assert_eq!(1, Solution::find_min(vec![3,4,5,1,2]));
        assert_eq!(0, Solution::find_min(vec![4,5,6,7,0,1,2]));
        assert_eq!(11, Solution::find_min(vec![11,13,15,17]));
        assert_eq!(1, Solution::find_min(vec![2,1]));
    }
}

