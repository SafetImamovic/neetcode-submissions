#include <unordered_map>

class Solution
{
        public:
        vector<int> topKFrequent(vector<int>& nums, int k)
        {
                vector<std::pair<int, int>> result;
                vector<int> real;
                std::unordered_map<int, int> map;

                for (int i = 0; i < nums.size(); i++)
                {
                        if (map.contains(nums.at(i)))
                        {
                                int &count = map.at(nums.at(i));
                                count++;
                        }
                        else
                        {
                                map.insert({nums.at(i), 1});
                        }
                }

                for (const auto &[key, value] : map)
                {
                        std::cout << key << ": " << value << '\n';
                }

                for (const auto &[key, value] : map)
                {
                        result.push_back({value, key});
                }

                sort(result.rbegin(), result.rend());
                result.resize(k);

                for (const auto &[value, key] : result)
                {
                        real.push_back(key);
                }

                return real;
        }
};