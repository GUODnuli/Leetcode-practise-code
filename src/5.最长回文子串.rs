/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 *
 * https://leetcode.cn/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (38.11%)
 * Likes:    7102
 * Dislikes: 0
 * Total Accepted:    1.6M
 * Total Submissions: 4.3M
 * Testcase Example:  '"babad"'
 *
 * 给你一个字符串 s，找到 s 中最长的回文子串。
 * 
 * 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "babad"
 * 输出："bab"
 * 解释："aba" 同样是符合题意的答案。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "cbbd"
 * 输出："bb"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 1000
 * s 仅由数字和英文字母组成
 * 
 * 
 */

// @lc code=start
// use crate::solution::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 { return s; }

        let s = s.chars().collect::<Vec<_>>();
        let mut start = 0;
        let mut max_len = 1;
        let mut dp = vec![vec![false; len]; len];

    
        for r in 1..len {
            for l in 0..r {
                if s[l] == s[r] && (r - l <= 2 || dp[l + 1][r - 1]) {
                    dp[]
                }
            }
        }

        s[start..start + max_len].iter().collect()
    }
}
// @lc code=end
