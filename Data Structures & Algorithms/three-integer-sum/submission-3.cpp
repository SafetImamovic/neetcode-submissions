class Solution
{
public:
        vector<vector<int>> threeSum(vector<int>& nums)
        {
                std::sort(nums.begin(), nums.end());

                std::vector<std::vector<int>> result;
                size_t n = nums.size();

                for (int i = 0; i < n; i++)
                {
                        size_t l = i + 1;
                        size_t r = n - 1;

                        if (i > 0 && nums[i] == nums[i - 1])
                        {
                                continue;
                        }

                        while (l < r)
                        {
                                // -4, -1, -1, 0, 1, 2
                                //  ^   ^            ^
                                //  i   l            r
                                // -4 + -1 + 2 = -3

                                // -4, -1, -1, 0, 1, 2
                                //  ^       ^        ^
                                //  i       l        r
                                // -4 + -1 + 2 = -3
                                int suma = nums[i] + nums[l] + nums[r];

                                if (suma > 0)
                                {
                                        r--;
                                }
                                else if (suma < 0)
                                {
                                        l++;
                                }
                                else
                                {
                                        result.push_back({nums[i], nums[l], nums[r]});

                                        l++;
                                        r--;

                                        while (l < r && nums[r] == nums[r + 1]) r--;
                                        while (l < r && nums[l] == nums[l - 1]) l++;    
                                        
                                }
                        }
                }

                return result;
        }
};
