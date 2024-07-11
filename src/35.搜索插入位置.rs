/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 *
 * https://leetcode.cn/problems/search-insert-position/description/
 *
 * algorithms
 * Easy (46.54%)
 * Likes:    2332
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 3.2M
 * Testcase Example:  '[1,3,5,6]\n5'
 *
 * 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
 * 
 * 请必须使用时间复杂度为 O(log n) 的算法。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 5
 * 输出: 2
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 2
 * 输出: 1
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: nums = [1,3,5,6], target = 7
 * 输出: 4
 * 
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums 为 无重复元素 的 升序 排列数组
 * -10^4 <= target <= 10^4
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return 0; }
        let (mut start, mut end) = (0usize, nums.len() - 1);
        let mut mid = 0;

        while start < end {
            mid = start + ((end - start) >> 1);
            if nums[mid] < target {
                if mid == nums.len() - 1 {
                    break;
                }
                start = mid + 1;
            } else if nums[mid] > target {
                if mid == 0 {
                    end = mid;
                    break;
                }
                end = mid - 1;
            } else {
                return mid as i32;
            }
        }

        if nums[end] >= target { return end as i32; }
        else { return (end + 1) as i32; }
    }
}
// @lc code=end

