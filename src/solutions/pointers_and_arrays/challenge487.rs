/*
Given a binary array nums, return the maximum number of consecutive 1's in the array if you can flip at most one 0.

 

Example 1:

Input: nums = [1,0,1,1,0]
Output: 4
Explanation: 
- If we flip the first zero, nums becomes [1,1,1,1,0] and we have 4 consecutive ones.
- If we flip the second zero, nums becomes [1,0,1,1,1] and we have 3 consecutive ones.
The max number of consecutive ones is 4.


*/

use std::cmp::max;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut left, mut right, mut current_zeros, mut answer) = (0, 0, 0 , 0);
        let n = nums.len();

        loop {
            if right == n {
                break;
            }

            if nums[right] == 0 {
                current_zeros += 1;
            }

            // we can have a max of one in a window
            while current_zeros == 2 {
                if nums[left] == 0 {
                    current_zeros -= 1;
                }
                left += 1;
            }
            
            answer = max(answer, right - left + 1);
            right += 1; 
        } 

        answer as i32
    }
}