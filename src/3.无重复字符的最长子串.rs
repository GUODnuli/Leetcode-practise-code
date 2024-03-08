/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (39.56%)
 * Likes:    10015
 * Dislikes: 0
 * Total Accepted:    2.7M
 * Total Submissions: 6.9M
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: s = "abcabcbb"
 * 输出: 3 
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: s = "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: s = "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= s.length <= 5 * 10^4
 * s 由英文字母、数字、符号和空格组成
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;
use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut max_len = 0usize;

        for (i, c) in s.char_indices() {
            if let Some(_value) = map.get(&c) {
                loop {
                    if let Some(element) = queue.pop_front() {
                        map.remove(&element);
                        if element == c { break; }
                    };
                }
                max_len = update_len(&mut queue, &mut map, c, i, max_len);
            } else {
                max_len = update_len(&mut queue, &mut map, c, i, max_len);
            }
        }
        max_len as i32
    }
}

pub fn update_len(queue: &mut VecDeque<char>, map: &mut HashMap<char, usize>, c: char, i: usize, max_len: usize) -> usize {
    queue.push_back(c);
    map.insert(c, i);
    if queue.len() > max_len {
        return queue.len();
    }
    max_len
}
// @lc code=end
