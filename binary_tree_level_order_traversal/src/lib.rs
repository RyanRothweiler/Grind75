use std::{cell::RefCell, rc::Rc};

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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut q = std::collections::VecDeque::new();
    let mut ret = Vec::new();

    match root {
        Some(r) => q.push_back(r),
        None => {
            return vec![];
        }
    }

    while !q.is_empty() {
        let mut row = Vec::new();
        let row_len = q.len();

        for _ in 0..row_len {
            let n = q.pop_front();
            match n {
                Some(node) => {
                    row.push(node.borrow().val);

                    if let Some(l) = node.borrow_mut().left.take() {
                        q.push_back(l);
                    }
                    if let Some(l) = node.borrow_mut().right.take() {
                        q.push_back(l);
                    }
                }
                None => continue,
            }
        }

        ret.push(row);
    }

    return ret;
}
