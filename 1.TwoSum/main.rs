// 1. Two Sum
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]
 
// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
 

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();

        result.sort_by(|a,b| a.1.cmp(&b.1));

        for i in 0..result.len() {
            let complement = target - result[i].1;
            let index = result[i+1..].binary_search_by(|x| x.1.cmp(&complement))
                .map(|x| x + i + 1)
                .or_else(|_| result[..i].binary_search_by(|x| x.1.cmp(&complement)));
            if index.is_ok() {
                return vec![result[i].0 as i32, result[index.unwrap()].0 as i32];
            }
        }
        vec![]
    }
}