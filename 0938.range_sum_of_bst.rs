#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
  pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut sum = 0;
    match root {
      Some(node) => {
        let node = node.borrow();

        if low <= node.val && node.val <= high {
          sum += node.val;
        }

        if node.val > low {
          sum += Self::range_sum_bst(node.left.clone(), low, high);
        }

        if node.val < high {
          sum += Self::range_sum_bst(node.right.clone(), low, high);
        }
      }
      None => {}
    }
    sum
  }
}