use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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
            right: None,
        }
    }
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(v) => {
            let mut new_node = TreeNode::new(v.borrow().val);

            let left = v.borrow().left.clone();
            let right = v.borrow().right.clone();

            new_node.left = invert_tree(right);
            new_node.right = invert_tree(left);

            return Some(Rc::new(RefCell::new(new_node)));
        }

        // Base case, do nothing
        None => {
            return None;
        }
    }
}
