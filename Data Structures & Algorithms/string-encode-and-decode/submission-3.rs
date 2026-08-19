impl Solution
{
        pub fn encode(strs: Vec<String>) -> String
        {
                let mut result: String = String::new();

                for i in &strs
                {
                        result.push_str(&i);
                        result.push_str("|-|");
                }

                println!("\"{}\"", &result);

                result
        }

        pub fn decode(s: String) -> Vec<String>
        {
                let mut result: Vec<String> = Vec::new();

                result = s
                         .split("|-|")
                         .map(|str| str.to_string())
                         .collect();

                result.pop();
                result
        }
}
