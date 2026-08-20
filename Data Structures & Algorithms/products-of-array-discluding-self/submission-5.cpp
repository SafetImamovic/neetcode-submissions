class Solution
{
        public:
        vector<int> productExceptSelf(vector<int>& nums)
        {
                std::vector<int> result(nums.size(), 1);

                for (int i = 1; i < nums.size(); i++)
                {
                        result.at(i) = result.at(i - 1) * nums.at(i - 1);
                }

                int postfix = 1;

                for (int i = nums.size() - 1; i >= 0; i--)
                {
                        result.at(i) *= postfix;
                        postfix *= nums.at(i);
                }

                return result;
        }
};