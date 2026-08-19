use std::collections::HashMap;

impl Solution
{
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>
        {
                let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

                for s in strs
                {
                        let mut key = [0u8; 26];

                        for b in s.bytes()
                        {
                                key[(b - b'a') as usize] += 1;
                        }

                        groups.entry(key).or_default().push(s);
                }

                groups.into_values().collect()
        }
}