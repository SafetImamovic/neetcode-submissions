impl Solution
{
        pub fn eval_rpn(tokens: Vec<String>) -> i32
        {
                let mut stack = Vec::new();

                for token in tokens
                {
                        match token.as_str()
                        {
                                "+" | "-" | "*" | "/" => {
                                        let b = stack.pop().unwrap();
                                        let a = stack.pop().unwrap();

                                        let result = match token.as_str()
                                        {
                                                "+" => a + b,
                                                "-" => a - b,
                                                "*" => a * b,
                                                "/" => a / b,
                                                _ => unreachable!(),
                                        };

                                        stack.push(result);
                                },
                                _ => stack.push(token.parse().unwrap()),
                        }
                }

                stack.pop().unwrap()
        }
}