/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 *
 * https://leetcode.cn/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (43.83%)
 * Likes:    3076
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 2.9M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 * 
 * 如果不存在公共前缀，返回空字符串 ""。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：strs = ["flower","flow","flight"]
 * 输出："fl"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：strs = ["dog","racecar","car"]
 * 输出：""
 * 解释：输入不存在公共前缀。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] 仅由小写英文字母组成
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;
use std::cmp::min;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut input: Vec<Vec<char>> = Vec::new();
        let mut min_len = 201;
        let mut ans = Vec::new();

        for i in strs.clone() {
            min_len = min(min_len, i.len());
            input.push(i.chars().collect());
        }

        'outer: for i in 0..min_len {
            ans.push(input[0][i]);
            for j in 0..input.len() {
                if input[j][i] != ans[i] {
                    ans.pop();
                    break 'outer;
                }
            }
        }
        
        ans.into_iter().collect()
    }
}
// @lc code=end

