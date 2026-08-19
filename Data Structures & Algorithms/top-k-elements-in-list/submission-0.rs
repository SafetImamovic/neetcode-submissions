use std::collections::HashMap;

impl Solution
{
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32>
    {
        let mut map = HashMap::new();

        for n in nums
        {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut entries = map.into_iter().collect::<Vec<_>>();

        entries.sort_by_key(|&(_, count)| count);
        entries.reverse();

        entries
            .into_iter()
            .take(k as usize)
            .map(|(number, _)| number)
            .collect()
    }
}