struct Stack<T> {
    stack: Vec<T>, 
}

impl<T> Stack<T> {

    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T>{
        self.stack.last()
    }
}

fn main() {
    let mut stack: Stack<isize> = Stack::new(); 
    stack.push(1); 
    stack.push(7); 
    let ans = stack.length(); 
    println!("{}", ans); 
}