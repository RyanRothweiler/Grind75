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

#[derive(Eq, PartialEq)]
enum SignalResult {
    Unbalanced,
    MaxDepth(i32),
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> SignalResult {
        match root {
            Some(r) => {
                let left_result = check(r.borrow().left.clone(), depth + 1);
                let right_result = check(r.borrow().right.clone(), depth + 1);

                if left_result == SignalResult::Unbalanced
                    || right_result == SignalResult::Unbalanced
                {
                    return SignalResult::Unbalanced;
                }

                match (left_result, right_result) {
                    (SignalResult::MaxDepth(left_v), SignalResult::MaxDepth(right_v)) => {
                        let dif: i32 = (left_v - right_v).abs();
                        if dif > 1 {
                            return SignalResult::Unbalanced;
                        } else {
                            return SignalResult::MaxDepth(std::cmp::max(left_v, right_v));
                        }
                    }

                    // We check if either signal is unbalanced before the match.
                    // So guaranteed both to be DepthCounter
                    // This can only happen if a new enum entry is added.
                    _ => panic!("this shouldn't happen ,comment"),
                }
            }

            // At the end of the tree
            // Sub one because this is not actually a node, so it doesn't add depth
            None => return SignalResult::MaxDepth(depth - 1),
        }

        // return SignalResult::Unbalanced;
    }

    match check(root, 0) {
        // Some point of the tree is unbalanced
        SignalResult::Unbalanced => return false,

        SignalResult::MaxDepth(_v) => return true,
    }
}
