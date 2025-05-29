use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

//  Definition for a binary tree node.
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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return i32::max(
                dfs(node.borrow().left.clone()) + 1,
                dfs(node.borrow().right.clone()) + 1,
            );
        } else {
            0
        }
    }

    dfs(root) + 1
}
