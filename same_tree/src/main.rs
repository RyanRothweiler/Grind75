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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                if p_node.borrow().val != q_node.borrow().val {
                    false
                } else {
                    return dfs(p_node.borrow().left.clone(), q_node.borrow().left.clone())
                        && dfs(p_node.borrow().right.clone(), q_node.borrow().right.clone());
                }
            }
        }
    }

    dfs(p, q)
}
