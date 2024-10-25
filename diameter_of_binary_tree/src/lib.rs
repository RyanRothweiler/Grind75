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

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    struct Ret {
        len: i32,
        potential_max: i32,
    }

    fn length(root: Option<Rc<RefCell<TreeNode>>>) -> Ret {
        match root {
            Some(v) => {
                let left_len = length(v.borrow().left.clone());
                let right_len = length(v.borrow().right.clone());

                let max = std::cmp::max(
                    left_len.len + right_len.len,
                    std::cmp::max(right_len.potential_max, left_len.potential_max),
                );

                return Ret {
                    len: std::cmp::max(left_len.len, right_len.len) + 1,
                    potential_max: max,
                };
            }
            None => {
                return Ret {
                    len: 0,
                    potential_max: 0,
                }
            }
        }
    }

    let ret = length(root);
    return ret.potential_max;
}
