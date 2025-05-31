use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

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
    struct ReturnPack {
        length: i32,
        current_max: i32,
    }

    fn length(root: Option<Rc<RefCell<TreeNode>>>) -> ReturnPack {
        if let Some(node) = root {
            let left = length(node.borrow().left.clone());
            let right = length(node.borrow().right.clone());

            let self_dia = left.length + right.length;
            let sides_dia_max = i32::max(left.current_max, right.current_max);
            let new_max = i32::max(sides_dia_max, self_dia);

            ReturnPack {
                length: i32::max(left.length, right.length) + 1,
                current_max: new_max,
            }
        } else {
            ReturnPack {
                length: 0,
                current_max: 0,
            }
        }
    }

    length(root).current_max
}
