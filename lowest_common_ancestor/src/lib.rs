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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // unwrap safe because problem guaranteed valid
    let p_val = p.unwrap().borrow().val;
    let q_val = q.unwrap().borrow().val;

    return check(root, p_val, q_val);
}

pub fn check(
    root: Option<Rc<RefCell<TreeNode>>>,
    p_val: i32,
    q_val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root.clone() {
        Some(rc) => {
            let rc_node = rc.borrow();
            let self_val = rc_node.val;

            // one is one left and the other is on the right, then we're the common
            if (p_val <= self_val && q_val >= self_val) || (p_val >= self_val && q_val <= self_val)
            {
                return root;
            }

            // both are on the left, then common is somewhere on the left
            if p_val < self_val && q_val < self_val {
                return check(rc_node.left.clone(), p_val, q_val);
            }

            // both are on the right, then common is somewhere on the left
            if p_val > self_val && q_val > self_val {
                return check(rc_node.right.clone(), p_val, q_val);
            }

            panic!("Should never happen because problem is guarandeed valid");
        }

        // this shoudn't ever happen because the problem is guaranteed valid.
        None => return None,
    }
}
