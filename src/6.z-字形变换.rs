/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 *
 * https://leetcode.cn/problems/zigzag-conversion/description/
 *
 * algorithms
 * Medium (52.62%)
 * Likes:    2283
 * Dislikes: 0
 * Total Accepted:    653.9K
 * Total Submissions: 1.2M
 * Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
 * 
 * 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：
 * 
 * 
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 
 * 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。
 * 
 * 请你实现这个将字符串进行指定行数变换的函数：
 * 
 * 
 * string convert(string s, int numRows);
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "PAYPALISHIRING", numRows = 3
 * 输出："PAHNAPLSIIGYIR"
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "PAYPALISHIRING", numRows = 4
 * 输出："PINALSIGYAHRPI"
 * 解释：
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "A", numRows = 1
 * 输出："A"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * s 由英文字母（小写和大写）、',' 和 '.' 组成
 * 1 
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 { return s; }
        
        let s = s.chars().collect::<Vec<_>>();
        let mut ans = String::new();
        let mut index = Vec::new();
        let len = s.len();
        let (mut i, mut row) = (0, 1);
        let mut back = false;

        while i < len {
            index.push(row);
            i += 1;
            match back {
                false => {
                    if row < num_rows {
                        row += 1;
                    } else {
                        row -= 1;
                        back = true;
                    }
                },
                true => {
                    if row > 1 {
                        row -= 1;
                    } else {
                        row += 1;
                        back = false;
                    }
                }
            }
        }
        
        for j in 1..=num_rows {
            for (charindex, num) in index.iter().enumerate() {
                if *num == j {
                    ans.push(s[charindex]);
                }
            }
        }

        ans
    }
}
// @lc code=end

