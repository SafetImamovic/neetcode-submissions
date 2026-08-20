impl Solution
{
        pub fn product_except_self(nums: Vec<i32>) -> Vec<i32>
        {
                let len = nums.len();
                let mut result = vec![1; len];

                let mut prefix = vec![1; len];
                let mut postfix = vec![1; len];

                for i in 1..len
                {
                        prefix[i] = prefix[i - 1] * nums[i - 1];
                }

                for i in (0..len - 1).rev()
                {
                        postfix[i] = postfix[i + 1] * nums[i + 1];
                }

                for i in 0..len
                {
                        result[i] = prefix[i] * postfix[i];
                }

                /*
                println!("{:#?}", &prefix);
                println!("{:#?}", &postfix);
                println!("{:#?}", &result);
                */

                result
        }
}
