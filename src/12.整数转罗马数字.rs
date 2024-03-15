/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 *
 * https://leetcode.cn/problems/integer-to-roman/description/
 *
 * algorithms
 * Medium (66.88%)
 * Likes:    1256
 * Dislikes: 0
 * Total Accepted:    457.2K
 * Total Submissions: 683.5K
 * Testcase Example:  '3'
 *
 * 罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。
 * 
 * 
 * 字符          数值
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * 
 * 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V +
 * II 。
 * 
 * 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数
 * 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
 * 
 * 
 * I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
 * X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。 
 * C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
 * 
 * 
 * 给你一个整数，将其转为罗马数字。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: num = 3
 * 输出: "III"
 * 
 * 示例 2:
 * 
 * 
 * 输入: num = 4
 * 输出: "IV"
 * 
 * 示例 3:
 * 
 * 
 * 输入: num = 9
 * 输出: "IX"
 * 
 * 示例 4:
 * 
 * 
 * 输入: num = 58
 * 输出: "LVIII"
 * 解释: L = 50, V = 5, III = 3.
 * 
 * 
 * 示例 5:
 * 
 * 
 * 输入: num = 1994
 * 输出: "MCMXCIV"
 * 解释: M = 1000, CM = 900, XC = 90, IV = 4.
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let roman = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        let mut ans = String::new();

        while num != 0 {
            match num {
                1000..=3999 => {
                    let times = num / 1000;
                    num = num % 1000;
                    for _ in 0..times {
                        ans.push(roman[6]);
                    }
                },
                900..=999 => {
                    num -= 900;
                    ans.push(roman[4]);
                    ans.push(roman[6]);
                },
                500..=899 => {
                    num -= 500;
                    ans.push(roman[5]);
                },
                400..=499 => {
                    num -= 400;
                    ans.push(roman[4]);
                    ans.push(roman[5]);
                },
                100..=399 => {
                    let times = num / 100;
                    num = num % 100;
                    for _ in 0..times {
                        ans.push(roman[4]);
                    }
                },
                90..=99 => {
                    num -= 90;
                    ans.push(roman[2]);
                    ans.push(roman[4]);
                },
                50..=89 => {
                    num -= 50;
                    ans.push(roman[3]);
                },
                40..=49 => {
                    num -= 40;
                    ans.push(roman[2]);
                    ans.push(roman[3]);
                },
                10..=39 => {
                    let times = num / 10;
                    num = num % 10;
                    for _ in 0..times {
                        ans.push(roman[2]);
                    }
                },
                9 => {
                    num -= 9;
                    ans.push(roman[0]);
                    ans.push(roman[2]);
                },
                5..=8 => {
                    num -= 5;
                    ans.push(roman[1]);
                },
                4 => {
                    num -= 4;
                    ans.push(roman[0]);
                    ans.push(roman[1]);
                },
                1..=3 => {
                    let times = num;
                    num = 0;
                    for _ in 0..times {
                        ans.push(roman[0]);
                    }
                },
                _ => {}
            }
        }

        ans
    }
}
// @lc code=end

