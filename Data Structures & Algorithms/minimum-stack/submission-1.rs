struct MinStack
{
        pub vec: Vec<i32>
}

impl MinStack
{
        pub fn new() -> Self
        {
                Self {
                        vec: Vec::new()
                }
        }

        pub fn push(&mut self, val: i32)
        {
                self.vec.push(val);
        }

        pub fn pop(&mut self)
        {
                self.vec.pop();
        }

        pub fn top(&self) -> i32
        {   
                self.vec.last().unwrap().clone()
        }

        pub fn get_min(&self) -> i32
        {
                self.vec.iter().min().unwrap().clone()
        }
}
