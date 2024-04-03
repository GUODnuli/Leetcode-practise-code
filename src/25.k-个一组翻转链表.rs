/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
 *
 * https://leetcode.cn/problems/reverse-nodes-in-k-group/description/
 *
 * algorithms
 * Hard (67.84%)
 * Likes:    2301
 * Dislikes: 0
 * Total Accepted:    578.8K
 * Total Submissions: 853.2K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。
 * 
 * k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
 * 
 * 你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：head = [1,2,3,4,5], k = 2
 * 输出：[2,1,4,3,5]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 
 * 
 * 输入：head = [1,2,3,4,5], k = 3
 * 输出：[3,2,1,4,5]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 链表中的节点数目为 n
 * 1 <= k <= n <= 5000
 * 0 <= Node.val <= 1000
 * 
 * 
 * 
 * 
 * 进阶：你可以设计一个只用 O(1) 额外内存空间的算法解决此问题吗？
 * 
 * 
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut remain = head;
        let mut result = ListNode::new(0);
        let mut tail = &mut result;
        let mut node_vec = Vec::new();

        'first_loop: while let Some(mut curr_node) = remain {
            let mut index = k - 1;
            remain = curr_node.next.take();
            node_vec.push(curr_node);
            while index != 0 {
                if let Some(mut curr_node) = remain {
                    index -= 1;
                    remain = curr_node.next.take();
                    node_vec.push(curr_node);
                } else {
                    for i in node_vec.into_iter() {
                        tail.next = Some(i);
                        tail = tail.next.as_mut().unwrap();
                    }
                    break 'first_loop;
                }
            }
            while node_vec.len() != 0 {
                tail.next = node_vec.pop();
                tail = tail.next.as_mut().unwrap();
            }
        }

        result.next
    }
}
// @lc code=end

