impl Solution
{
        pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>
        {
                /*
                Important constraint is that index1 < index2.
                e.g. [1, 2];

                Example:
                numbers = [1,2,3,4], target = 3
                Return the 1-base indexed indices that add up
                to target such that index1 < index2.

                n = numbers.len();
                l = 0;
                r = n - 1;

                    [1,2,3,4]
                     ^     ^
                     l     r

                if numbers[r] >= target
                // 4 >= 3
                {
                        r -= 1;
                }

                    [1,2,3,4]
                     ^   ^
                     l   r

                if numbers[r] >= target
                // 3 > 3   <- Candidate for sum, we're looking for 0
                {
                        r -= 1;
                        continue;
                }

                if numbers[l] + numbers[r] > target // 1 + 3 = 4, 4 > 3
                {
                        r -= 1;
                }
                else if numbers[l] + numbers[r] < target 
                {
                        l += 1;
                }
                else
                {
                        return result;
                }
                        


                for i in 0..n
                {
                        
                }

                */
                
                let n: usize = numbers.len();          // Length of the input array.
                let mut l: usize = 0;                  // left pointer index.
                let mut r: usize = n - 1;              // right pointer index.

                for i in 0..n
                {
                        if numbers[l] + numbers[r] > target
                        {
                                r -= 1;
                        }
                        else if numbers[l] + numbers[r] < target 
                        {
                                l += 1;
                        }
                }

                let result: Vec<i32> = vec![l as i32 + 1, r as i32 + 1];
                result
        }
}
