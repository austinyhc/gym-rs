/**
 * [15] 3Sum
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
 * such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
 * Notice that the solution set must not contain duplicate triplets.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Explanation: 
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
 * The distinct triplets are [-1,0,1] and [-1,-1,2].
 * Notice that the order of the output and the order of the triplets does not matter.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [0,1,1]
 * Output: []
 * Explanation: The only possible triplet does not sum up to 0.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [0,0,0]
 * Output: [[0,0,0]]
 * Explanation: The only possible triplet sums up to 0.
 * 
 *  
 * Constraints:
 * 
 * 	3 <= nums.length <= 3000
 * 	-10^5 <= nums[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut nums = nums;
        nums.sort();

        let mut results = vec![];
        
        for k in 0..nums.len() {
            if nums[k] > 0 { break; }
            if k > 0 && nums[k-1] == nums[k] { continue; }

            let target = 0 - nums[k];
            for pair in Self::two_sum(nums[k+1..].to_vec(), target) {
                let mut result: Vec<i32> = vec![nums[k]];
                result.extend_from_slice(&pair);
                result.sort();
                results.push(result);
            }
        }
        results
    }

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut dict: HashMap<i32, bool> = HashMap::new();
        let mut results = vec![];

        for num in nums {
            let to_find = target - num;

            if let Some(_) = dict.get(&to_find) {
                let mut result = vec![num, to_find];
                result.sort();
                results.push(result);
            } else {
                dict.insert(num, true);
            }
        }
        results
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![vec![2,12],vec![5,9],vec![7,7]],
            Solution::two_sum(vec![2,3,2,7,9,12,5,7], 14));
    }

    #[test]
    fn test_15() {
        assert_eq!(vec![vec![-1,0,1], vec![-1,-1,2]],
            Solution::three_sum(vec![-1,0,1,2,-1,-4]));
        assert_eq!(Vec::<Vec<i32>>::new(),
            Solution::three_sum(vec![0,1,1]));
        assert_eq!(vec![vec![0,0,0]],
            Solution::three_sum(vec![0,0,0]));

    }
}

