/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (index, element) in nums.iter().enumerate() {
            if map.contains_key(&(target - element)) {
                return vec! [
                    *map.get_mut(&(target - element)).unwrap(),
                    index as i32
                ];
            } else {
                map.insert(*element, index as i32);
            }
        }
        Vec::new()
    }
}

// @lc code=end
