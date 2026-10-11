impl Solution
{
        pub fn is_palindrome(s: String) -> bool
        {
                /*
                There are 2 cases, even and odd number of chars.
                
                Two pointer solution: L and R.
                L starts from 0 up to the middle of the string.
                R starts from the end of the string up to the middle.
                At each iteration we check if the char is the same.
                If at one point they're not the same return false else true.
                 
                Example:
                    tab a cat
                    ^       ^
                    L       R -> L = t == R = t    ok
                
                    tab a cat
                     ^     ^
                     L     R  -> L = a == R = a    ok

                    tab a cat
                      ^   ^
                      L   R   -> L = b == R = c    not ok                     
                */

                let mut vec_s: Vec<char> = s.to_lowercase()
                                           .chars()
                                           .filter(|c| c.is_alphanumeric())
                                           .collect();

                let n: usize = vec_s.len();
                let half_n: usize = n / 2;
                let mut l: usize = 0;
                let mut r: usize = n - 1;
                
                /*
                It actually doesn't matter if the length of the
                string is even or odd, because we don't have the
                compare the middle char with any other char.

                We just need to iterate to half rounded down of
                the size of the array.

                for "tab a cat" -> 9 => 9/2 => 4, so we do 4 comparisons.
                */
                for i in 0..half_n
                {  
                        if vec_s[l] != vec_s[r]
                        {
                                return false;
                        }

                        l += 1;
                        r -= 1;
                }
                  
                true
        }
}
