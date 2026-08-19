impl Solution
{
        const MARKER: char = '#';

        pub fn encode(strs: Vec<String>) -> String
        {
                let mut result: String = String::new();

                for i in &strs
                {
                        let len = i.len();

                        result.push_str(&format!("{}{}{}", len, Self::MARKER, i));
                }

                result
        }

        pub fn decode(s: String) -> Vec<String>
        {
                let mut result: Vec<String> = Vec::new();
                let mut len: String = String::new();
                let mut len_u: usize = 0;
                let mut str_: String = String::new();
                let mut mode: bool = false; // false -> get length, true -> get strings.

                for i in s.bytes()
                {
                        if !mode
                        {
                                if i as char == Self::MARKER
                                {
                                        len_u = len.parse().expect("Couldn't parse!");
                                        len.clear();

                                        if len_u == 0
                                        {
                                                result.push(String::new());
                                                mode = false;
                                        }
                                        else
                                        {
                                                mode = true;
                                        }

                                        continue;
                                }

                                len.push(i as char);
                        }
                        else
                        {
                                if len_u > 0 
                                {
                                        str_.push(i as char);
                                        len_u -= 1;
                                }

                                if len_u == 0
                                {
                                        result.push(str_.clone());
                                        str_.clear();
                                        len.clear();

                                        mode = false;
                                }
                        }
                }

                result
        }
}
