impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool
        {
                if s.len() != t.len()
                {
                        return false;
                }

                const a: u8 = 97;

                let mut map: [i32; 26] = [0i32; 26];

                for c in s.bytes() {
                        map[(c - a) as usize] += 1;
                }

                for c in t.bytes() {
                        map[(c - a) as usize] -= 1;
                }

                map.iter().all(|&count| count == 0)
        }
}