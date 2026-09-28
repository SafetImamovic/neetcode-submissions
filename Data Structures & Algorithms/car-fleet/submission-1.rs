impl Solution
{
        pub fn tta(target: i32, pos: i32, speed: i32) -> f64
        {
                (target - pos) as f64 / speed as f64
        }

        pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32
        {
                let n: usize = position.len();
                let mut pos_vel: Vec<(i32, i32)> = vec![];
                let mut stack: Vec<(i32, i32)> = vec![];

                for i in 0..n
                {
                        let pos = position[i];
                        let vel = speed[i];
                        pos_vel.push((pos, vel));
                }

                pos_vel.sort();
                pos_vel.reverse();

                for i in 0..n
                {
                        if stack.is_empty()
                        {
                                stack.push(pos_vel[i]);
                                continue;
                        }

                        let top = *stack.last().unwrap();

                        stack.push(pos_vel[i]);

                        let top_until = Self::tta(target, top.0, top.1);
                        let until =     Self::tta(target, pos_vel[i].0, pos_vel[i].1);

                        if until <= top_until
                        {
                                stack.pop();
                        }
                }
 
                stack.len() as i32
        }
}
