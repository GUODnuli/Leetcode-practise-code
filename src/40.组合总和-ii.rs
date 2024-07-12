/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 *
 * https://leetcode.cn/problems/combination-sum-ii/description/
 *
 * algorithms
 * Medium (59.46%)
 * Likes:    1560
 * Dislikes: 0
 * Total Accepted:    541.5K
 * Total Submissions: 910.7K
 * Testcase Example:  '[10,1,2,7,6,1,5]\n8'
 *
 * 给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 * 
 * candidates 中的每个数字在每个组合中只能使用 一次 。
 * 
 * 注意：解集不能包含重复的组合。 
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: candidates = [10,1,2,7,6,1,5], target = 8,
 * 输出:
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 * 
 * 示例 2:
 * 
 * 
 * 输入: candidates = [2,5,2,1,2], target = 5,
 * 输出:
 * [
 * [1,2,2],
 * [5]
 * ]
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 <= candidates.length <= 100
 * 1 <= candidates[i] <= 50
 * 1 <= target <= 30
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        let mut candidates = candidates.clone();
        candidates.sort_unstable();
        Self::backtrack_40(&candidates, target, &mut vec![], &mut result, 0, 0);

        result.into_iter().collect()
    }
    fn backtrack_40(candidates: &Vec<i32>, target: i32, curr_vec: &mut Vec<i32>, result: &mut HashSet<Vec<i32>>, start_index: usize, current_sum: i32) {
        if current_sum == target {
            result.insert(curr_vec.clone());
        }

        for i in start_index..candidates.len() {
            // 跳过重复元素
            if i > start_index && candidates[i] == candidates[i - 1] {
                continue;
            }

            let new_sum = current_sum + candidates[i];
            //提前终止搜索
            if new_sum > target {
                break;
            }

            curr_vec.push(candidates[i]);
            Self::backtrack_40(candidates, target, curr_vec, result, i + 1, new_sum);
            curr_vec.pop(); // 回溯时移除最后一个元素
        }
    }
}
// @lc code=end