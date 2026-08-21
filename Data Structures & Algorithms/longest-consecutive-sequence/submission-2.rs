
// @leet start
use std::collections::HashSet;

impl Solution
{
        pub fn longest_consecutive(nums: Vec<i32>) -> i32
        {
                let mut set: HashSet<i32> = HashSet::new();

                for i in nums
                {
                        set.insert(i);
                }

                let mut longest = 0;
                let mut temp_longest = 0;

                for i in &set
                {
                        // for 100:
                        // if set contains 99
                        // continue;
                        if set.contains(&(i - 1))
                        {
                                temp_longest = 1;

                                continue;
                        }

                        // doesn't contain 99 so:
                        // while set contains 100 + longest
                        while set.contains(&(i + temp_longest))
                        {
                                temp_longest += 1;
                        }

                        if longest <= temp_longest
                        {
                                longest = temp_longest;
                        }
                }


                longest
        }
}
// @leet end

