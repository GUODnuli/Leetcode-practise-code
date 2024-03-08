/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (42.15%)
 * Likes:    29840
 * Dislikes: 5844
 * Total Accepted:    4.3M
 * Total Submissions: 10.1M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.

use crate::solution::Solution;

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

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let (mut node1, mut node2, mut carry) = (l1.as_deref(), l2.as_deref(), 0);
        let mut current = dummy_head.as_mut();
        
        while node1.is_some() || node2.is_some() {
            let (sum, new_carry) = add(node1.map_or(0, |v| v.val), node2.map_or(0, |v| v.val), carry);
            carry = new_carry;
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            current = current.unwrap().next.as_mut();
            node1 = node1.and_then(|v| v.next.as_deref());
            node2 = node2.and_then(|v| v.next.as_deref());
        }
        
        if carry > 0 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        
        dummy_head.unwrap().next
    }
}

pub fn add(num1: i32, num2: i32, carry: i32) -> (i32, i32) {
    let sum = num1 + num2 + carry;
    return (sum % 10, sum / 10);
}
// @lc code=end
