/**
 * [104] Maximum Depth of Binary Tree
 *
 * Given the root of a binary tree, return its maximum depth.
 * A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/tmp-tree.jpg" style="width: 400px; height: 277px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 3
 * 
 * <strong class="example">Example 2:
 * 
 * Input: root = [1,null,2]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-100 <= Node.val <= 100
 * 
 */
pub struct Solution {}
use crate::utils::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/maximum-depth-of-binary-tree/
// discuss: https://leetcode.com/problems/maximum-depth-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = i32::MIN;
        Self::max_depth_helper(root.as_ref(), 0, &mut max_depth);
        max_depth
    }

    fn max_depth_helper(node: Option<&Rc<RefCell<TreeNode>>>,
                            depth: i32, max_depth: &mut i32) {
        match node {
            None => {
                if depth > *max_depth {
                    *max_depth = depth;
                    return;
                }
            },
            Some(n) => {
                Self::max_depth_helper(n.borrow().left.as_ref(), depth+1, max_depth);
                Self::max_depth_helper(n.borrow().right.as_ref(), depth+1, max_depth);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_104() {
        assert_eq!(Solution::max_depth(tree![]), 0);
        assert_eq!(Solution::max_depth(tree![3, 9, 20, null, null, 15, 7]), 3);
    }
}

