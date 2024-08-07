/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 *
 * https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (47.54%)
 * Likes:    2834
 * Dislikes: 0
 * Total Accepted:    1.4M
 * Total Submissions: 2.9M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：head = [1,2,3,4,5], n = 2
 * 输出：[1,2,3,5]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：head = [1], n = 1
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：head = [1,2], n = 1
 * 输出：[1]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 链表中结点的数目为 sz
 * 1 <= sz <= 30
 * 0 <= Node.val <= 100
 * 1 <= n <= sz
 * 
 * 
 * 
 * 
 * 进阶：你能尝试使用一趟扫描实现吗？
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
  fn _new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
use crate::solution::Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
      let mut dummy = Some(Box::new(ListNode{ val: 0, next: head }));
      let mut slow_p = &mut dummy;
      let mut fast_p = &slow_p.clone();

      for _ in 0..=n {
        if let Some(fast_node) = fast_p {
          fast_p = &fast_node.next;
        } else {
          return None;
        }
      }

      while fast_p.is_some() {
        fast_p = &fast_p.as_ref().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
      }

      let remove_p = &mut slow_p.as_mut().unwrap().next;
      slow_p.as_mut().unwrap().next = remove_p.as_mut().unwrap().next.take();

      dummy.unwrap().next
    }
}
// @lc code=end
