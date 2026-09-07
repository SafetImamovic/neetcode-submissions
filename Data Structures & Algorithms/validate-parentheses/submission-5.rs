use std::collections::HashMap;

impl Solution
{
        pub fn is_valid(s: String) -> bool
        {
                let mut pairs: HashMap<char, char> = HashMap::from([
                        ('(', ')'),
                        ('{', '}'),
                        ('[', ']'),
                        ]);

                let mut stack: Vec<char> = Vec::new();
                
                for brace in s.chars()
                {
                        if stack.is_empty()
                        {
                                stack.push(brace);
                                continue;
                        }
                        
                        // we're getting the top of the stack, suppose '('
                        let top = stack.last().unwrap();

                        // we're getting the pair for the top of the stack, so ')'
                        match pairs.get(&top)
                        {
                                Some(pair) => {
                                        // if the new character `brace` is the same as the pair for '(' -> ')'
                                        if &pair == &&brace
                                        {
                                                // Then we pop from the stack.
                                                stack.pop();
                                                continue;
                                        }
                                },
                                None => {}
                        }

                        stack.push(brace);
                }

                println!("{:#?}", &stack);

                if stack.is_empty()
                {
                        return true;
                }

                false
        }
}
