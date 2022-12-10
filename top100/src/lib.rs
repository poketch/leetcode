#![allow(unused_variables)]
mod twosum;
mod valid_parentheses;
mod merge_two_lists;
mod search_insert_position;
mod climbing_stairs;
mod binary_tree_inorder_traversal;
mod pascals_triangle;
mod best_time_to_buy_and_sell_stock;
mod single_number;
mod majority_element;
mod move_zeroes;
mod longest_substring_without_repeating_numbers;

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>,
  pub right: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


