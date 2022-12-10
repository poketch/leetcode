// 94 - Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/

use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        // Inorder Traversal means Presenting Left -> Node -> Right going through every sub node using that same presenteation

        // e.g. Reading the Tree left to right [1 , 2, 3, 6, 7, 4, 5]

        // Inorder reading is: [6, 2, 7, 1, 4, 3, 5]
        
        if root.is_none() {
            return vec![];
        }

        let node = Rc::clone(&root.unwrap());

        let left = node.borrow_mut().left.as_ref();
        let right = node.borrow_mut().right.as_ref();
        
        

        
        vec![]
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::TreeNode;

    #[test]
    fn ex1() {
        let node = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        };

        assert_eq!(
            crate::Solution::inorder_traversal(Some(Rc::new(RefCell::new(node)))),
            vec![1, 3, 2]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::inorder_traversal(None), vec![])
    }

    #[test]
    fn ex3() {
        let node = TreeNode::new(1);

        assert_eq!(
            crate::Solution::inorder_traversal(Some(Rc::new(RefCell::new(node)))),
            vec![1]
        )
    }

    #[test]
    fn ex4() {
        let node = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
        };

        assert_eq!(
            crate::Solution::inorder_traversal(Some(Rc::new(RefCell::new(node)))),
            vec![1,5,3,6,2,7,4,8]
        )
    }

    #[test]
    fn ex5() {
        let node = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        right:  Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 6,
                        left:  Some(Rc::new(RefCell::new(TreeNode::new(11)))),
                        right:  Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(16)))),
                    }))),
                }))),
            }))),
        };

        assert_eq!(
            crate::Solution::inorder_traversal(Some(Rc::new(RefCell::new(node)))),
            vec![17,1,9,5,10,3,11,6,12,2,13,7,14,4,15,8,16]
        )
    }
}
