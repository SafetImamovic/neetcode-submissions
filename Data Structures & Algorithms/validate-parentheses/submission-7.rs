impl Solution
{
        pub fn is_valid(s: String) -> bool
        {
                let mut stack: Vec<u8> = Vec::new();

                for brace in s.bytes()
                {
                        match brace
                        {
                                b'(' | b'{' | b'[' => stack.push(brace),
                                b')' =>
                                {
                                        if stack.pop() != Some(b'(')
                                        {
                                                return false;
                                        }
                                },
                                b'}' =>
                                {
                                        if stack.pop() != Some(b'{')
                                        {
                                                return false;
                                        }
                                },
                                b']' =>
                                {
                                        if stack.pop() != Some(b'[')
                                        {
                                                return false;
                                        }
                                },
                                _ => {return false;}
                        }
                }

                stack.is_empty()
        }
}
