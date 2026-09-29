class Solution
{
public:
        float tta(int &target, int &pos, int &time)
        {
                return (target - pos) / (float)time;
        }

        int carFleet(int target, vector<int>& position, vector<int>& speed)
        {
                // position.size() == speed.size() == n
                std::size_t n = position.size();
                std::vector<std::pair<int, int>> pos_vel; // position_velocity
                std::vector<std::pair<int, int>> stack;

                for (int i = 0; i < n; i++)
                {
                        // First pair element is the position.
                        // Second is the velocity (speed).
                        // Constructed from the &position and &speed vectors.
                        std::pair<int, int> pos_vel_pair{position[i], speed[i]};
                        pos_vel.push_back(pos_vel_pair);
                }

                // Sorting in descending order.
                std::sort(pos_vel.begin(), pos_vel.end(), [&](std::pair<int, int> a, std::pair<int, int> b)
                {
                        return a.first > b.first;
                });

                for (std::pair<int, int> &i : pos_vel)
                {
                        if (stack.size() == 0)
                        {
                                stack.push_back(i);
                                continue;
                        }

                        // Ensured that it's not empty.
                        std::pair<int, int> top = stack.back();

                        // We push the next car onto the stack.
                        stack.push_back(i);

                        // Getting the tta (time to arrival) for the car at
                        // the top of the stack and the next descending car.
                        float until_top = tta(target, top.first, top.second);
                        float until     = tta(target, i.first, i.second);

                        // If the arrival time of the latest car is greater than
                        // the arrival time of the car being compared to
                        // That means that it will form a fleet meaning that 
                        // we don't have to compare the latest car with the next car
                        // because the car at the top of the stack has a slower or equal
                        // speed.
                        if (until <= until_top)
                        {
                                stack.pop_back();
                        }
                }

                return stack.size();
        }
};
