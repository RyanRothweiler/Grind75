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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn valid(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            Some(n) => {
                let node = n.borrow();
                let nv = node.val as i64;

                if nv > min && nv < max {
                    let l = valid(n.borrow().left.clone(), min, nv);
                    let r = valid(n.borrow().right.clone(), nv, max);
                    return l && r;
                } else {
                    return false;
                }
            }
            None => return true,
        }
    }

    return valid(root, i64::MIN, i64::MAX);
}
