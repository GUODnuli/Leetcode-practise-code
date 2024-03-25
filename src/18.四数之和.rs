/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 *
 * https://leetcode.cn/problems/4sum/description/
 *
 * algorithms
 * Medium (36.64%)
 * Likes:    1885
 * Dislikes: 0
 * Total Accepted:    566.6K
 * Total Submissions: 1.5M
 * Testcase Example:  '[1,0,-1,0,-2,2]\n0'
 *
 * 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a],
 * nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
 * 
 * 
 * 0 <= a, b, c, d < n
 * a、b、c 和 d 互不相同
 * nums[a] + nums[b] + nums[c] + nums[d] == target
 * 
 * 
 * 你可以按 任意顺序 返回答案 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,0,-1,0,-2,2], target = 0
 * 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [2,2,2,2,2], target = 8
 * 输出：[[2,2,2,2]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 200
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 4 {
            return vec![];
        }
        let mut result = Vec::new();
        let mut nums = nums;
        nums.sort();
        let mut map = HashMap::new();
        for i in 0..len {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..len {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let sum = nums[i] + nums[j];
                map.entry(sum).or_insert(Vec::new()).push((i, j));
            }
        }
        let mut set = HashSet::new();
        for i in 0..len {
            for j in i + 1..len {
                let sum = nums[i] + nums[j];
                if let Some(pairs) = map.get(&(target - sum)) {
                    for &(x, y) in pairs {
                        if x != i && x != j && y != i && y != j {
                            let mut temp = vec![nums[i], nums[j], nums[x], nums[y]];
                            temp.sort();
                            set.insert(temp);
                        }
                    }
                }
            }
        }

        'outside: for item in set {
            let mut sum: i64 = 0;
            for j in &item {
                let max = std::i32::MAX as i64;
                let min = std::i32::MIN as i64;
                sum += *j as i64;
                if sum > max || sum < min {
                    continue 'outside; 
                }
            }
            result.push(item);
        }
        result
    }
}
// @lc code=end

