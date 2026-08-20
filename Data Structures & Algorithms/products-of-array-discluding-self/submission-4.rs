impl Solution
{
        pub fn product_except_self(nums: Vec<i32>) -> Vec<i32>
        {
                let len = nums.len();
                let mut result = vec![1; len];

                for i in 1..len
                {
                        result[i] = result[i - 1] * nums[i - 1];
                }

                let mut postfix = 1;

                for i in (0..len).rev()
                {
                        result[i] *= postfix;
                        postfix *= nums[i];
                }

                /*
                println!("{:#?}", &prefix);
                println!("{:#?}", &postfix);
                println!("{:#?}", &result);
                */

                result
        }
}
