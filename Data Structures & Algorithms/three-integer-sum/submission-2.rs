impl Solution
{
        pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>>
        {       
                /*
                The result is a vector of triplets such that: nums[i] + nums[j] + nums[k] == 0
                Constraints:
                        1. i != j != k.
                        2. No duplicate triplets.
                        3. Order doesn't matter.
                        4. Total number of elements in the result <= nums.
                        5. Elements can appear multiple times in the result.

                Example:
                        nums = [-1,0,1,2,-1,-4]

                        result = [[-1,-1,2],[-1,0,1]]

                        [-1,0,1,2,-1,-4]
                          ^ ^         ^
                          i l         r -> i + l + r = -5

                        [-1,0,1,2,-1,-4]
                          ^   ^       ^
                          i   l       r -> i + l + r = -4

                        [-1,0,1,2,-1,-4]
                          ^     ^     ^
                          i     l     r -> i + l + r = -3

                        [-1,0,1,2,-1,-4]
                          ^        ^  ^
                          i        l  r -> i + l + r = -6

                        [-1,0,1,2,-1,-4]
                          ^ ^      ^
                          i l      r    -> i + l + r = -2
                
                        [-1,0,1,2,-1,-4]
                          ^ ^   ^
                          i l   r       -> i + l + r = 1

                        [-1,0,1,2,-1,-4]
                          ^ ^ ^
                          i l r         -> i + l + r = 0
                        
                */

                nums.sort();
                let mut result: Vec<Vec<i32>> = vec![];
                let n: usize = nums.len();

                for i in 0..n
                {
                        let mut l = i + 1;
                        let mut r = n - 1;

                        if i > 0 && nums[i] == nums[i - 1]
                        {
                                continue;
                        }

                        while l < r
                        {
                                let sum: i32 = nums[i] + nums[l] + nums[r];

                                if sum > 0
                                {
                                        r -= 1;
                                }
                                else if sum < 0
                                {
                                        l += 1;
                                }
                                else
                                {
                                        result.push(vec![nums[i], nums[l], nums[r]]);

                                        l += 1;
                                        r -= 1;

                                        while l < r && nums[l] == nums[l - 1]
                                        {
                                                l += 1;
                                        }

                                        while l < r && nums[r] == nums[r + 1]
                                        {
                                                r -= 1;
                                        }
                                }
                        }
                }

                result
        }
}
