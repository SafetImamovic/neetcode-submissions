impl Solution
{
        pub fn eval_rpn(tokens: Vec<String>) -> i32
        {
                let mut stack: Vec<i32> = vec![];

                for token in tokens
                {
                        match token.as_str()
                        {
                                "+" | "-" | "*" | "/" =>
                                {
                                        let first:  i32 = stack.pop().unwrap();
                                        let second: i32 = stack.pop().unwrap();

                                        let result = match token.as_str()
                                        {
                                                "+" => first + second,
                                                "-" => second - first,
                                                "*" => first * second,
                                                "/" => second / first,
                                                _ => unreachable!(),
                                        };

                                        stack.push(result);
                                },
                                _ => {stack.push(token.parse().unwrap())}
                        } 
                }

                stack.pop().unwrap()
        }
}