/*

905. Sort Array By Parity

Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.

Return any array that satisfies this condition.

 

Example 1:

Input: nums = [3,1,2,4]
Output: [2,4,3,1]
Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
Example 2:

Input: nums = [0]
Output: [0]


 */


 impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        let (mut start, mut end) = (0, nums.len() - 1);

        while start < end {
            if nums[start] % 2 == 1 && nums[end] % 2 == 0 {
                let temp = nums[start];
                nums[start] = nums[end];
                nums[end] = temp;
            }

            if nums[start]%2 == 0 {
                start += 1;
            }

            if nums[end]%2 == 1 {
                end -= 1;
            }
        }
        nums
    }
}