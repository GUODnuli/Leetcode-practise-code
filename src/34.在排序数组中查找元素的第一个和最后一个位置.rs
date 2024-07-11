/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 *
 * https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/description/
 *
 * algorithms
 * Medium (43.68%)
 * Likes:    2728
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 2.3M
 * Testcase Example:  '[5,7,7,8,8,10]\n8'
 *
 * 给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。
 * 
 * 如果数组中不存在目标值 target，返回 [-1, -1]。
 * 
 * 你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。
 * 
 * 
 * 
 * 示例 1：
 * 
 *
 * 输入：nums = [5,7,7,8,8,10], target = 8
 * 输出：[3,4]
 *
 * 示例 2：
 *
 *
 * 输入：nums = [5,7,7,8,8,10], target = 6
 * 输出：[-1,-1]
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [], target = 0
 * 输出：[-1,-1]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * nums 是一个非递减数组
 * -10^9 <= target <= 10^9
 * 
 *
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        if nums.is_empty() { return result; }

        let (mut start, mut end) = (0usize, nums.len() - 1);
        let mut mid = start + ((end - start) >> 1);
        while start <= end {
            mid = start + ((end - start) >> 1);
            if nums[mid] < target {
                if mid == nums.len() { break; }
                start = mid + 1;
            } else if nums[mid] > target {
                if mid == 0 { break; }
                end = mid - 1;
            } else {
                let (mut result_start, mut result_end) = (mid, mid);
                while result_start > 0 && nums[result_start] == target {
                    result_start -= 1;
                }
                while result_end < nums.len() - 1 && nums[result_end] == target {
                    result_end += 1;
                }
                if nums[result_start] == target { result[0] = result_start as i32; }
                else { result[0] = (result_start + 1) as i32; }
                if nums[result_end] == target { result[1] = result_end as i32; }
                else { result[1] = (result_end - 1) as i32; }
                break;
            }
        }
        result
    }
}
// @lc code=end
