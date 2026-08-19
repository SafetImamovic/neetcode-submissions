use std::collections::HashMap;

impl Solution
{
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>
        {
                // nums[i] + nums[j] = target
                // nums[i] = target - nums[j]
                let mut map: HashMap<i32, i32> = HashMap::new();
                let mut result: Vec<i32> = Vec::with_capacity(2);

                for (i, num) in nums.iter().enumerate()
                {
                        if map.contains_key(&(target - num))
                        {
                                result.push(*map.get(&(target - num)).unwrap());
                                result.push(i as i32);
                        }

                        map.insert(*num, i as i32);
                }

                result
        }
}