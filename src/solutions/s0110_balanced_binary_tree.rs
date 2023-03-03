/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is <span data-keyword="height-balanced">height-balanced</span>.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg" style="width: 342px; height: 221px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg" style="width: 452px; height: 301px;" />
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 * 
 * <strong class="example">Example 3:
 * 
 * Input: root = []
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-10^4 <= Node.val <= 10^4
 * 
 */
pub struct Solution {}
use crate::utils::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/balanced-binary-tree/
// discuss: https://leetcode.com/problems/balanced-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let left_h = Self::height(node.borrow().left.clone());
            let right_h = Self::height(node.borrow().right.clone());

            return ((left_h - right_h).abs() < 2) &&
                    Self::is_balanced(node.borrow().left.clone()) &&
                    Self::is_balanced(node.borrow().right.clone());
        } 
        true
    }

    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return 1 + std::cmp::max(
                Self::height(node.borrow().left.clone()),
                Self::height(node.borrow().right.clone()));
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        assert_eq!(2, Solution::height(tree!(3,9,20,null,null,15,7)));

        assert_eq!(true, Solution::is_balanced(tree!(3,9,20,null,null,15,7)));
        assert_eq!(false, Solution::is_balanced(tree!(1,2,2,3,3,null,null,4,4)));
        assert_eq!(true, Solution::is_balanced(tree!()));
        assert_eq!(true, Solution::is_balanced(tree!(1,2,3,4,5,6,null,8)));
        assert_eq!(false, Solution::is_balanced(tree!(1,2,2,3,null,null,3,4,null,null,4)));
    }
}

