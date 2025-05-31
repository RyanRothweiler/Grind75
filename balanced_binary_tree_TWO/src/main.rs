use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

enum Balanced {
    Yes { max_depth: i32 },
    No,
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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    #[derive(Eq, PartialEq)]
    enum Balanced {
        Yes { max_depth: i32 },
        No,
    }

    fn check(root: Option<Rc<RefCell<TreeNode>>>) -> Balanced {
        if let Some(node) = root {
            let left_bal = check(node.borrow().left.clone());
            let right_bal = check(node.borrow().right.clone());

            if right_bal == Balanced::No || left_bal == Balanced::No {
                return Balanced::No;
            }

            match (left_bal, right_bal) {
                (
                    Balanced::Yes {
                        max_depth: left_val,
                    },
                    Balanced::Yes {
                        max_depth: right_val,
                    },
                ) => {
                    let dif = (left_val - right_val).abs();
                    if dif > 1 {
                        Balanced::No
                    } else {
                        Balanced::Yes {
                            max_depth: i32::max(left_val, right_val) + 1,
                        }
                    }
                }

                _ => panic!("This should never happen"),
            }
        } else {
            Balanced::Yes { max_depth: 0 }
        }
    }

    match check(root) {
        Balanced::Yes { max_depth: _ } => true,
        Balanced::No => false,
    }
}
