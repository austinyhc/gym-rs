/**
 * [46] Permutations
 *
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * <strong class="example">Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * <strong class="example">Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 6
 * 	-10 <= nums[i] <= 10
 * 	All the integers of nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations/
// discuss: https://leetcode.com/problems/permutations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len == 1 {
            return vec![nums];
        }
        let mut results = vec![];
        for i in 0..len {
            let mut cloned = nums.clone();
            let num = cloned.remove(i);
            for v in Self::permute(cloned).iter_mut() {
                v.push(num);
                results.push(v.clone());
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
    fn test_46() {
        let test = vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],
                        vec![2,3,1],vec![3,1,2],vec![3,2,1]];
        let result = Solution::permute(vec![1,2,3]);
        assert!(test.iter().all(|e| result.contains(e)));


        let test = vec![vec![0,1],vec![1,0]];
        let result = Solution::permute(vec![0,1]);
        assert!(test.iter().all(|e| result.contains(e)));

        assert_eq!(vec![vec![1]], Solution::permute(vec![1]));
    }
}

