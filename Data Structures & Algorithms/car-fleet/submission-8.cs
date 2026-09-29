public class Solution
{
        public float tta(int target, int pos, int vel)
        {
                return (target - pos) / (float)vel;
        }

        public int CarFleet(int target, int[] position, int[] speed)
        {
                int n = position.Length;
                (int, int)[] pos_vel = new (int, int)[n];
                Stack<(int, int)> stack = new Stack<(int, int)>();

                for (int i = 0; i < n; i++)
                {
                        pos_vel[i] = ((position[i], speed[i]));
                }

                Array.Sort(pos_vel);
                // Before this I had pos_vel.Reverse() it didn't do anything.
                // That led me to believe that Array.Reverse(pos_val) was the right 
                // thing to do because it's probably a static function but it didn't tell me that
                // meaning that if it is a static function it's definition that doesn't require 
                // any paramaters is weird right?
                Array.Reverse(pos_vel);

                foreach((int pos, int vel) in pos_vel)
                {
                        Console.WriteLine($"{pos}, {vel}");

                        if (stack.Count == 0)
                        {
                                stack.Push((pos, vel));
                        }

                        // The stack isn't emtpy here.
                        (int, int) top = stack.Peek();

                        stack.Push((pos, vel));

                        float until_top = tta(target, top.Item1, top.Item2);
                        float until     = tta(target, pos, vel);

                        if (until_top >= until)
                        {
                                stack.Pop();
                        }
                }

                return stack.Count;
        }
}
