impl Solution
{
        pub fn search(nums: Vec<i32>, target: i32) -> i32
        {
                if nums.is_empty()
                {
                        return -1;
                }

                let mut l: usize = 0;
                let mut r: usize = nums.len() - 1;

                while l <= r
                {
                        let mid = l + (r - l) / 2;

                        if nums[mid] == target
                        {
                                return mid as i32;
                        }

                        if nums[mid] > target
                        {
                                if mid == 0
                                {
                                        break;
                                }

                                r = mid - 1;
                        }
                        else
                        {
                                l = mid + 1;
                        }
                }

                -1
        }
}