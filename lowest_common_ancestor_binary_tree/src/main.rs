#![allow(clippy::all, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

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
    fn check(
        root: Option<Rc<RefCell<TreeNode>>>,
        q_val: i32,
        p_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let bind = root.clone().unwrap();
        let r = bind.borrow();
        if r.val == q_val || r.val == p_val {
            return root;
        }

        let left = check(r.left.clone(), q_val, p_val);
        let right = check(r.right.clone(), q_val, p_val);

        // One node is on right and one is on left
        if left.is_some() && right.is_some() {
            return root;
        } else if left.is_some() {
            return left;
        } else if right.is_some() {
            return right;
        }

        None
    }

    let p_val = p.unwrap().borrow().val;
    let q_val = q.unwrap().borrow().val;

    check(root, q_val, p_val)
}
