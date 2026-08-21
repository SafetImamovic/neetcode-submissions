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

                for i in &set
                {
                        // Not the beginning of a sequence.
                        if set.contains(&(i - 1))
                        {
                                continue;
                        }

                        let mut temp_longest = 1;

                        while set.contains(&(i + temp_longest))
                        {
                                temp_longest += 1;
                        }

                        if longest < temp_longest
                        {
                                longest = temp_longest;
                        }
                }

                longest
        }
}