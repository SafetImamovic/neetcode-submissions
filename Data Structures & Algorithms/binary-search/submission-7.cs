public class Solution
{
        public int Search(int[] nums, int target)
        {
                int l = 0;
                int r = nums.Length - 1;

                while (l <= r)
                {
                        int mid = l + (r - l) / 2;

                        if (nums[mid] == target)
                        {
                                return mid;
                        }

                        if (nums[mid] > target)
                        {
                                r = mid - 1;
                        }
                        else if (nums[mid] < target)
                        {
                                l = mid + 1;
                        }
                }

                return -1;
        }
}
