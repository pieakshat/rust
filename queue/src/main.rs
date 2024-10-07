use std::fmt::Debug; // Import the Debug trait

struct Queue<T> {
    queue: Vec<T>, 
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0) // This removes and returns the first item
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    fn return_queue(&self) -> Vec<T>
    where
        T: Clone, // Require that T implements Clone
    {
        self.queue.clone() 
    }

}

fn main() {
    let mut q: Queue<isize> = Queue::new();
    q.enqueue(1);
    q.enqueue(7);
    let i = q.dequeue(); 
    q.enqueue(5); 
    q.enqueue(10); 
    let ans = q.return_queue();
    assert_eq!(i, 1);
    assert_eq!(q.is_empty(), false);
    let a = q.peek(); 

    // Debug printing the queue and peek value
    println!("Queue after dequeue: {:?}", ans); 
    println!("Peek value: {:?}", a); // This will print the peek value
    

}
