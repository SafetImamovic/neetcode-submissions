struct MinStack
{
        core: Vec<i32>,
        min: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack 
{
        fn new() -> Self
        {
                Self
                {
                        core: vec![],
                        min: vec![],
                }
        }
        
        fn push(&mut self, value: i32)
        {
                self.core.push(value);

                if self.min.is_empty()
                {
                        self.min.push(value);
                        return;
                }

                if &value > self.min.last().unwrap()
                {
                        return;
                }

                self.min.push(value);
        }
        
        fn pop(&mut self)
        {
                let pop = self.core.pop().unwrap();

                if self.min.is_empty()
                {
                        return;
                }

                if &pop == self.min.last().unwrap()
                {
                        self.min.pop();
                }
        }
        
        fn top(&self) -> i32
        {
                self.core.last().unwrap_or(&0).clone()
        }
        
        fn get_min(&self) -> i32
        {
                self.min.last().unwrap_or(&0).clone()
        }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(value);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */