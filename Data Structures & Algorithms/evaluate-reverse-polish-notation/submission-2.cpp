class Solution
{
public:
        int evalRPN(vector<string>& tokens)
        {
                std::vector<int> stack = {};

                for (const string &token : tokens)
                {
                        if (token == "+" || token == "-" || token == "*" || token == "/")
                        {
                                int first = stack.back();
                                stack.pop_back();

                                int second = stack.back();
                                stack.pop_back();

                                int result = 0;

                                if (token == "+")
                                {
                                        result = second + first;
                                }
                                else if (token == "-")
                                {
                                        result = second - first;
                                }
                                else if (token == "*")
                                {
                                        result = second * first;
                                }
                                else if (token == "/")
                                {
                                        result = second / first;       
                                }

                                stack.push_back(result);
                        }
                        else
                        {
                                stack.push_back(std::stoi(token));
                        } 
                }

                return stack.back();
        }
};