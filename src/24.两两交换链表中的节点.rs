/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 *
 * https://leetcode.cn/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (72.01%)
 * Likes:    2184
 * Dislikes: 0
 * Total Accepted:    819.1K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：head = [1,2,3,4]
 * 输出：[2,1,4,3]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：head = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：head = [1]
 * 输出：[1]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 链表中节点的数目在范围 [0, 100] 内
 * 0 <= Node.val <= 100
 * 
 * 
 */

// @lc code=start
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
use crate::solution::Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut remain = head;
        let mut result = ListNode::new(0);
        let mut tail = &mut result;
        while let Some(mut n1) = remain {
          remain = n1.next.take();
          if let Some(mut n2) = remain {
            remain = n2.next.take();
            n2.next = Some(n1);
            tail.next = Some(n2);
            tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
          } else {
            tail.next = Some(n1);
            tail = tail.next.as_mut().unwrap();
          }
        }
        result.next
    }
}
// @lc code=end

