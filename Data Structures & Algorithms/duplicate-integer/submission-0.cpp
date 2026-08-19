class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        unordered_set<int> hash_set;

        for (int i = 0; i < nums.size(); i++)
        {
                if (hash_set.count(nums[i]))
                        return true;

                hash_set.insert(nums[i]);
        }

        return false;
    }
};
