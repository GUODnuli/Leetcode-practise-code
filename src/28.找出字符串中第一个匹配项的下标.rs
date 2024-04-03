/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 找出字符串中第一个匹配项的下标
 *
 * https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
 *
 * algorithms
 * Easy (43.53%)
 * Likes:    2191
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 2.4M
 * Testcase Example:  '"sadbutsad"\n"sad"'
 *
 * 给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0
 * 开始）。如果 needle 不是 haystack 的一部分，则返回  -1 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：haystack = "sadbutsad", needle = "sad"
 * 输出：0
 * 解释："sad" 在下标 0 和 6 处匹配。
 * 第一个匹配项的下标是 0 ，所以返回 0 。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：haystack = "leetcode", needle = "leeto"
 * 输出：-1
 * 解释："leeto" 没有在 "leetcode" 中出现，所以返回 -1 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= haystack.length, needle.length <= 10^4
 * haystack 和 needle 仅由小写英文字符组成
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (h_len, n_len) = (haystack.len(), needle.len());
        if h_len < n_len {
            return -1;
        }
        if h_len == n_len {
            if haystack == needle {
                return 0;
            } else {
                return -1;
            }
        }

        let (haystack, needle): (Vec<char>, Vec<char>) = (haystack.chars().collect(), needle.chars().collect());

        for (i, c) in haystack.iter().enumerate() {
            if *c == needle[0] {
                let (mut h_index, mut n_index) = (i, 0);
                while h_index < h_len && haystack[h_index] == needle[n_index] {
                    n_index += 1;
                    h_index += 1;
                    if n_index == n_len {
                        return i as i32;
                    }
                }
            }
        }

        -1
    }
}
// @lc code=end

